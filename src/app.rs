use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use winit::application::ApplicationHandler;
use winit::event::*;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::wgpu_ctx::WgpuCtx;
use crate::player;

#[derive(Default)]
pub struct App<'window> {
    dt_start: Option<Instant>,
    dt: Duration,
    player: player::Player,
    keys: HashMap<u8, bool>,
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'window>>,
}

impl App<'_> {
    pub fn init(&mut self) {
        self.player = player::Player::new();
        self.player.speed = 2.0;
    }
}

impl<'window> ApplicationHandler for App<'window> {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("Rust Terrain Generation");
            let window = Arc::new(event_loop.create_window(win_attr).expect("Error creating window."));
            self.window = Some(window.clone());
            self.wgpu_ctx = Some(WgpuCtx::new(window.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let (Some(wgpu_ctx), Some(window)) =
                    (self.wgpu_ctx.as_mut(), self.window.as_ref())
                {
                    wgpu_ctx.resize((new_size.width, new_size.height));
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
                    if self.dt_start.is_some() {
                        self.dt = self.dt_start.unwrap().elapsed();
                    }
                    else {
                        self.dt = Duration::new(0, 0);
                    }
                    self.dt_start = Some(Instant::now());
                    
                    wgpu_ctx.update(self.dt, &(self.player));
                    
                    wgpu_ctx.draw();
                    self.player.update(&self.keys, self.dt.as_secs_f32());
                }
            }
            WindowEvent::KeyboardInput {device_id: _, event, is_synthetic} => {
                if !is_synthetic {
                    if event.logical_key.to_text().is_some() {
                        self.keys.insert((event.logical_key.to_text().unwrap().as_bytes()[0]) as u8, event.state == ElementState::Pressed);
                    }
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_mut().unwrap().request_redraw();
    }
}