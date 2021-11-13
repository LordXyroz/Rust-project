use gl;

pub struct Mesh {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
}

impl Mesh {
    pub fn new() -> Mesh { 
        let mut mesh: Mesh = Mesh {
            vao: 0,
            vbo: 0,
        };

        unsafe {
            gl::GenVertexArrays(1, &mut mesh.vao);
            gl::GenBuffers(1, &mut mesh.vbo);
        }

        return mesh;
    }

    pub fn bind_vector_data(&self, data: Vec<f32>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::BindVertexArray(self.vao);

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
    }

    pub fn draw_mesh(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}