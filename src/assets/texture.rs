use gl;
use std::collections::HashMap;

pub struct Texture {
    id: u16,
    name: String,
    filepath: String,
}

pub struct TextureArray {
    name_to_id: HashMap<String, u16>,
    gl_id: gl::types::GLuint,
}
