use gl;
use std::collections::HashMap;

pub type TextureID = u16;
pub const TEXTURE_NOT_FOUND_ID: TextureID = 0;

pub struct Texture {
    id: TextureID,
    name: String,
    filepath: String,
}

pub struct TextureArray {
    name_to_id: HashMap<String, TextureID>,
    gl_id: gl::types::GLuint,
}

impl TextureArray {
    pub fn get_id(&self, name: &str) -> TextureID {
        if let Some(id) = self.name_to_id.get(name) {
            return id.clone();
        } else {
            return TEXTURE_NOT_FOUND_ID;
        }
    }
}
