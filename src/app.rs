
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
struct MouseGrabber {
	last_pos: winit::dpi::PhysicalPosition<f64>,
	manual_lock: bool,
}

// winit does not support cursor locking on win10, so we need to make it ourselves

impl MouseGrabber {
	fn cursor_moved(&mut self, window: &Window, pos: winit::dpi::PhysicalPosition<f64>) {
		if self.manual_lock {
			window.set_cursor_position(self.last_pos).unwrap();
		} else {
			self.last_pos = pos;
		}
	}
    
	fn grab(&mut self, window: &Window, grab: bool) {
		if grab {
			if window.set_cursor_grab(winit::window::CursorGrabMode::Locked).is_err() {
				window.set_cursor_grab(winit::window::CursorGrabMode::Confined).unwrap();
				self.manual_lock = true;
			}
		} else {
			self.manual_lock = false;
			window.set_cursor_grab(winit::window::CursorGrabMode::None).unwrap();
		}
		window.set_cursor_visible(!grab);
	}
}

#[derive(Default)]
pub struct App<'window> {
    dt_start: Option<Instant>,
    dt: Duration,
    player: player::Player,
    keys: HashMap<u8, bool>,
    window: Option<Arc<Window>>,
    wgpu_ctx: Option<WgpuCtx<'window>>,
    mouse_grabber: MouseGrabber
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
            self.mouse_grabber.grab(&window, true);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
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
                    if event.logical_key.to_text().unwrap_or(&"") == "\x1b" {
                        self.mouse_grabber.grab(self.window.as_mut().unwrap(), false);
                    }
                    if event.logical_key.to_text().is_some() {
                        self.keys.insert((event.logical_key.to_text().unwrap().as_bytes()[0]) as u8, event.state == ElementState::Pressed);
                    }
                }
            }
            WindowEvent::CursorMoved { device_id: _, position } => {
                self.mouse_grabber.cursor_moved(self.window.as_mut().unwrap(), position);
            }
            WindowEvent::MouseInput { device_id: _, state: _, button } => {
                if button == winit::event::MouseButton::Left {
                    self.mouse_grabber.grab(self.window.as_mut().unwrap(), true);
                }
            }
            _ => (),
        }
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                println!("{}, {}", delta.0, delta.1);
            }
            _ => ()
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_mut().unwrap().request_redraw();
    }

}