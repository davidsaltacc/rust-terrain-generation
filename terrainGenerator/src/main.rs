use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

fn main() {
    env_logger::init(); // logs errors when shit goes wrong
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| { //do on every event
        Event::WindowEvent {
            
        }
        match event { //look for specific event
            Event::WindowEvent {event: WindowEvent::CloseRequested, .. } => { //close window
                *control_flow = ControlFlow::Exit;
            },
            _ => *control_flow = ControlFlow::Wait, //wait for next event
        }
    });
}