use glium::glutin::{
    event_loop::EventLoop,
    window::WindowBuilder,
    ContextBuilder
};

pub struct Window{
    pub event_loop: EventLoop<()>,
    pub display: glium::Display
}

impl Window{

    pub fn create(title: &str) -> Window{

        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new().with_title(title);
        let cb = ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap(); 

        Window {
            event_loop: event_loop,
            display: display,
        }
    }
}
