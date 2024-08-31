
use crate::vector::Vector3;
use crate::vertex_data::{self, VertexData};
use crate::{transforms, utils};
use std::borrow::Cow;
use std::iter;
use std::sync::Arc;
use bytemuck::{Pod, Zeroable};
use cgmath::{ Matrix4, Point3};
use wgpu::util::DeviceExt;
use wgpu::MemoryHints::Performance;
use wgpu::ShaderSource;
use winit::window::Window;
use crate::player;


#[allow(unused)] // TODO remove this once it is used
pub struct WgpuContext<'window> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
    model_mat: Matrix4<f32>,
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
    vertex_buffer: wgpu::Buffer, 
    uniform_buffer: wgpu::Buffer,
    vertex_data: VertexData
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 4]
}

fn vertex(pos: [f32; 3]) -> Vertex {
    return Vertex {
        position: [pos[0], pos[1], pos[2], 1.0]
    };
}

fn create_vertices(pos: &Vec<[f32; 3]>) -> Vec<Vertex> {
    let mut vdata: Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        vdata.push(vertex(pos[i]));
    }
    return vdata.to_vec();
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x4];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES
        }
    }
}

impl<'window> WgpuContext<'window> {
    pub async fn new_async(window: Arc<Window>) -> WgpuContext<'window> {

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,

                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
        
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);

        //= Self::create_pipeline(&device, surface_config.format, width, height);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let model_mat = transforms::create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let view_mat = transforms::create_view((0., 0., 0.).into(), (0., 0., 0.).into(), cgmath::Vector3::unit_y());
        let project_mat = transforms::create_projection(width as f32 / height as f32, true);

        let mut uniforms = Vec::<f32>::new(); 
        for mat in [model_mat, view_mat, project_mat] {
            let mat_4x4: [[f32; 4]; 4] = mat.into();
            uniforms.extend(mat_4x4.concat());
        }
        uniforms.push(0.);
        uniforms.push(0.);
        uniforms.push(0.);
        uniforms.push(0.);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }],
            label: Some("Bind Group Layout")
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding()
            }],
            label: Some("Bind Group")
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format.into(),
                    blend: Some(wgpu::BlendState { 
                        color: wgpu::BlendComponent::REPLACE, 
                        alpha: wgpu::BlendComponent::REPLACE 
                    }),
                    write_mask: wgpu::ColorWrites::ALL
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None
        });

        let vertex_data = VertexData::new();
    
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&create_vertices(&vertex_data.positions)),
            usage: wgpu::BufferUsages::VERTEX,
        });

        return WgpuContext {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            render_pipeline,
            uniform_bind_group,
            model_mat,
            view_mat,
            project_mat,
            vertex_buffer,
            uniform_buffer,
            vertex_data
        };
    }

    pub fn new(window: Arc<Window>) -> WgpuContext<'window> {
        return pollster::block_on(WgpuContext::new_async(window));
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);

        self.project_mat = transforms::create_projection(width as f32 / height as f32, true);
    }

    pub fn update(&mut self, _dt: std::time::Duration, player: &player::Player) {

        let mut camera_look_direction: Vector3 = utils::rotation_to_direction(player.smooth_camera_rotation);
        camera_look_direction.x = -camera_look_direction.x;
        
        self.view_mat = transforms::create_view(Point3::new(0.0, 0.0, -0.0000001), Point3::from(camera_look_direction), cgmath::Vector3::unit_y());
        self.project_mat = transforms::create_projection(self.surface_config.width as f32 / self.surface_config.height as f32, true);
        self.model_mat = transforms::create_transforms(<[f32; 3]>::from(player.get_relative_position(Vector3::new(player.player_position.x - 2.5 * vertex_data::render_dist_mul(), -5., player.player_position.z - 2.5 * vertex_data::render_dist_mul()))), [0., 0., 0.], [1., 1., 1.]);

        let mut uniforms = Vec::<f32>::new(); 
        for mat in [self.model_mat, self.view_mat, self.project_mat] {
            let mat_4x4: [[f32; 4]; 4] = mat.into();
            uniforms.extend(mat_4x4.concat());
        }
        uniforms.push(player.player_position.x as f32);
        uniforms.push(player.player_position.z as f32);
        uniforms.push(0. as f32);
        uniforms.push(0. as f32);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&uniforms));
    }

    pub fn draw(&mut self) {
        let surface_texture = self.surface.get_current_texture().expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let multisample_texture = self.device.create_texture(&wgpu::TextureDescriptor{
            format: surface_texture.texture.format(),
            sample_count: 4,
            size: surface_texture.texture.size(),
            usage: surface_texture.texture.usage(),
            mip_level_count: surface_texture.texture.mip_level_count(),
            label: Some("Multisample Texture"),
            dimension: surface_texture.texture.dimension(),
            view_formats: &[]
        });
        let multisample_view = multisample_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.surface_config.width,
                height: self.surface_config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Depth Texture"),
            view_formats: &[]
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default() );

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Command Encoder") }); 
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &multisample_view,
                    resolve_target: Some(&texture_view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard
                    }),
                    stencil_ops: None
                }),
                timestamp_writes: None,
                occlusion_query_set: None
            });

            pass.set_pipeline(&self.render_pipeline);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            pass.draw(0..self.vertex_data.length, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        surface_texture.present();

    }
}

