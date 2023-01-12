use glam::{Vec2, Vec3};

#[repr(C, packed)]
pub struct Vertex {
    texture_diffuse: u32,
    texture_specular: u32,
    texture_glossiness: u32,
    position: Vec3,
    normal: Vec3,
    color: [u8; 4],
    uv: Vec2,
}

impl Vertex {
    pub fn new(
        texture_diffuse: u32,
        texture_specular: u32,
        texture_glossiness: u32,
        position: Vec3,
        normal: Vec3,
        color: [u8; 4],
        uv: Vec2,
    ) -> Self {
        Vertex {
            texture_diffuse,
            texture_specular,
            texture_glossiness,
            position,
            normal,
            color,
            uv,
        }
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
            ebo: 0,
        }
    }

    pub fn draw(&self) {
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32) }
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
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
        self.vbo = 0;
        self.vao = 0;
    }

    #[allow(unused_assignments)]
    pub fn load_without_ebo(&mut self) {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;

        let mut current_layout_index = 0;

        macro_rules! set_vertex_attrib_pointer {
            ($field: tt, $gl_type: expr, $count: expr) => {
                gl::EnableVertexAttribArray(current_layout_index);
                gl::VertexAttribPointer(
                    current_layout_index,         // index of the generic vertex attribute ("layout (location = 0)")
                    $count,         // the number of components per generic vertex attribute
                    $gl_type, // data type
                    gl::FALSE, // normalized (int-to-float conversion)
                    std::mem::size_of::<Vertex>() as gl::types::GLint, // stride (byte offset between consecutive attributes)
                    memoffset::offset_of!(Vertex, $field) as *const gl::types::GLvoid,  // offset of the first component
                );
                current_layout_index = current_layout_index + 1;
            };
        }

        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            set_vertex_attrib_pointer!(texture_diffuse, gl::UNSIGNED_INT, 1);
            set_vertex_attrib_pointer!(texture_specular, gl::UNSIGNED_INT, 1);
            set_vertex_attrib_pointer!(texture_glossiness, gl::UNSIGNED_INT, 1);
            set_vertex_attrib_pointer!(position, gl::FLOAT, 3);
            set_vertex_attrib_pointer!(normal, gl::FLOAT, 3);
            set_vertex_attrib_pointer!(color, gl::UNSIGNED_BYTE, 4);
            set_vertex_attrib_pointer!(uv, gl::FLOAT, 2);
        }

        self.unbind();

        self.vao = vao;
        self.vbo = vbo;
        self.loaded = true;
    }
}
