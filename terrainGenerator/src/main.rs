use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

fn main() {
    env_logger::init(); // logs errors when shit goes wrong
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                 event: WindowEvent::KeyboardInput { input, .. },
                 window_id,
             } if window_id == window.id() => {
                 if input.virtual_keycode == Some(VirtualKeyCode::Escape) { 
                     *control_flow = ControlFlow::Exit
                 } else if input.virtual_keycode == Some(VirtualKeyCode::W) {
                    *control_flow = ControlFlow::Exit
                 }
             }
            _ => (),
        }
    });
}