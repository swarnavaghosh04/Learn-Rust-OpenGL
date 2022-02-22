extern crate glium;
extern crate glutin;

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder
};

fn main(){

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Hello world!");
    let cb = ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &el).unwrap();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event{
            Event::WindowEvent{event, window_id} => {
                if window_id == display.gl_window().window().id(){
                    match event{
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::CursorEntered{device_id: _} => *control_flow = ControlFlow::Exit,
                        _ => ()
                    }
                }
            },
            _ => ()
        }
    });
}