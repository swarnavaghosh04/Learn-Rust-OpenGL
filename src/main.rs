#[macro_use]
extern crate glium;

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
#version 140

out vec4 color;

void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

fn main(){

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex{
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let data = [
        Vertex {position: [-0.75, -0.75]},
        Vertex {position: [0., 0.75]},
        Vertex {position: [0.75, -0.75]},
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    event_loop.run(move |event, _, control_flow| {

        let mut target = display.draw();
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