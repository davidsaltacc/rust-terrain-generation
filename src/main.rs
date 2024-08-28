use wgpu::{RequestAdapterOptions, SurfaceConfiguration};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    run();
}

fn run<'a>() {
    struct State<'a> {
        surface: wgpu::Surface<'a>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
        size: winit::dpi::PhysicalSize<u32>,
        window: Window,
    }

    impl<'a> State<'a> {
        fn new(window: Window) -> State<'a> {
            let size = window.inner_size();
    
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                #[cfg(not(target_arch="wasm32"))]
                backends: wgpu::Backends::PRIMARY,
                #[cfg(target_arch="wasm32")]
                backends: wgpu::Backends::GL,
                ..Default::default()
            });
            
            let surface = instance.create_surface(window).unwrap();
    
            let adapter = pollster::block_on(instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            )).unwrap();
       
            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor{
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: wgpu::MemoryHints::Performance,
                    label: None,
                },
                None,
            )).unwrap();

            let surface_caps = surface.get_capabilities(&adapter);

            let surface_format = surface_caps.formats.iter()
                .find(|f| f.is_srgb())
                .copied()
                .unwrap_or(surface_caps.formats[0]);

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            return Self {
                window,
                surface,
                device,
                queue,
                config,
                size,
            };
        }
        
        pub fn window(&self) -> &Window {
            &self.window
        }

        fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            todo!()
        }

        fn input(&mut self, event: &WindowEvent ) -> bool {
            todo!()
        }

        fn update(&mut self) {
            todo!()
        }

        fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
            todo!()
        }
    }

    let event_loop = EventLoop::new(); 
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let state = State::new(window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == state.window().id() => *control_flow = ControlFlow::Exit,
             _ => (),
            }
        }
    )
}