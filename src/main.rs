
#[macro_use]
extern crate glium;

mod support;

#[allow(unused_imports)]
use glium::{
    glutin::{
        self,
        event_loop::EventLoop
    },
    Surface
};

#[derive(Copy, Clone)]
struct Vertex{
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main(){

    let win = support::Window::create("Learn Rust Graphics");

    let data = [
        Vertex {position: [-0.75, -0.75]},
        Vertex {position: [0., 0.75]},
        Vertex {position: [0.75, -0.75]},
    ];

    let vertex_buffer = glium::VertexBuffer::new(&win.display, &data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vs_src = std::fs::read_to_string("share/learn-rust-graphics/vs.glsl")
        .expect("error occured while reading vertex shaer");
    let fs_src = std::fs::read_to_string("share/learn-rust-graphics/fs.glsl")
        .expect("error occured while reading fragment shaer");

    let program = glium::Program::from_source(&win.display, &vs_src, &fs_src, None).unwrap();

    win.event_loop.run(move |event, _, control_flow| {

        let mut target = win.display.draw();
        target.clear_color(0., 0., 0., 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        // *control_flow = ControlFlow::Wait;
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event{
            glutin::event::Event::WindowEvent{event, ..} => match event{
                glutin::event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                _ => ()
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => ()
        }
    });
}