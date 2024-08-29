// #![windows_subsystem = "windows"]
// NOTE, this removes all print statements, so only uncomment this when building as release

use crate::app::App;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

mod transforms;
mod app;
mod wgpu_ctx;
mod vertex_data;
mod player;
mod utils;
mod vector;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    app.init();
    return event_loop.run_app(&mut app);
}