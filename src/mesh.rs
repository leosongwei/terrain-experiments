use glam::{Vec2, Vec3};

#[repr(C, packed)]
pub struct Vertex {
    diffuse_index: u32,
    position: Vec3,
    color: [u8; 4],
    uv: Vec2
}

impl Vertex {
    pub fn new(
        diffuse_index: u32,
        position: Vec3,
        color: [u8; 4],
        uv: Vec2
    ) -> Self {
        Vertex { diffuse_index, position, color, uv }
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    loaded: bool,
    indexed: bool,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Mesh {
            vertices,
            loaded: false,
            indexed: false,
            vao: 0,
            vbo: 0,
            ebo: 0
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                self.vertices.len() as i32
            )
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn unload(&mut self) {
        log::debug!("unload mesh");
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
        self.vbo = 0;
        self.vao = 0;
    }

    pub fn load_without_ebo(&mut self) {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                1,         // the number of components per generic vertex attribute
                gl::UNSIGNED_INT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                std::mem::size_of::<Vertex>() as gl::types::GLint, // stride (byte offset between consecutive attributes)
                memoffset::offset_of!(Vertex, diffuse_index) as *const gl::types::GLvoid,  // offset of the first component
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,         // index 
                3,         // count
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                std::mem::size_of::<Vertex>() as gl::types::GLint, // stride
                memoffset::offset_of!(Vertex, position) as *const gl::types::GLvoid // offset
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,         // index 
                4,         // count
                gl::UNSIGNED_BYTE, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                std::mem::size_of::<Vertex>() as gl::types::GLint, // stride
                memoffset::offset_of!(Vertex, color) as *const gl::types::GLvoid // offset
            );

            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,         // index 
                2,         // count
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                std::mem::size_of::<Vertex>() as gl::types::GLint, // stride
                memoffset::offset_of!(Vertex, uv) as *const gl::types::GLvoid // offset
            );
        }

        self.unbind();

        self.vao = vao;
        self.vbo = vbo;
        self.loaded = true;
        log::debug!("vao: {vao}, vbo: {vbo}");
    }
}