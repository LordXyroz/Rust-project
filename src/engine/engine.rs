extern crate glfw;
extern crate gl;

use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Key};

use crate::engine::graphics;
use crate::engine::internal;
use crate::engine::internal::object_handler;

pub struct Engine {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

struct TestObj {
    transform: internal::transform::Transform,
}

impl object_handler::GameObject for TestObj {
    fn awake(&self) {
        println!("Obj awakened!");
    }

    fn start(&self) {
        println!("Obj started!");
    }

    fn update(&self) {
        println!("Obj updated!");
    }

    fn late_update(&self) {
        println!("Obj late updated!");
    }

    fn destroy(&self) {

    }

}

impl Engine {

    pub fn new() -> Engine {
        
        let _glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut _window, mut _events) = _glfw.create_window(1600, 900, "Hello GLFW from Rust!", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
    
        _window.make_current();
        _window.set_key_polling(true);

        gl::load_with(|s| _window.get_proc_address(s) as *const _);

        Engine {
            glfw: _glfw,
            window: _window,
            events: _events,
        }
    }

    pub fn engine_loop(&mut self) {
        let asd: TestObj = TestObj {transform: internal::transform::Transform::default()};
        
        object_handler::add_gameobject(asd);

        object_handler::awake_objects();
        object_handler::start_objects();

        use std::ffi::CString;
        
        let vert_shader = graphics::Shader::from_vert_source(&CString::new(include_str!("./graphics/shaders/triangle.vert")).unwrap()).unwrap();
        let frag_shader = graphics::Shader::from_frag_source(&CString::new(include_str!("./graphics/shaders/triangle.frag")).unwrap()).unwrap();

        let shader_program = graphics::ShaderProgram::from_shaders(&[vert_shader, frag_shader]).unwrap();

        shader_program.set_used();

        while !self.window.should_close() {
            object_handler::update_objects();
            object_handler::late_update_objects();
            // Swap front and back buffers
            self.window.swap_buffers();

            // Vertices
            let vertices: Vec<f32> = vec![
                -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 
                0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 
                0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
            ];

            // VBO
            let mut vbo: gl::types::GLuint = 0;
            unsafe {
                gl::GenBuffers(1, &mut vbo);

                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }

            //VAO
            let mut vao: gl::types::GLuint = 0;
            unsafe {
                gl::GenVertexArrays(1, &mut vao);
            }

            unsafe {
                gl::BindVertexArray(vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(
                    0,
                    3, 
                    gl::FLOAT,
                    gl::FALSE,
                    (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(
                    1, 
                    3, 
                    gl::FLOAT, 
                    gl::FALSE,
                    (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);
            }

            unsafe {
                gl::Viewport(0, 0, 1600, 900);
                gl::ClearColor(0.0, 0.3, 0.5, 1.0);

                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }

            // Poll for and process events
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&mut self.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }
}

