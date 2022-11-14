use gl;

pub trait GLObject {
    fn gl_id(&self) -> gl::types::GLuint;
    fn is_loaded_in_gl(&self) -> bool;
    fn load_into_gl(&self);
    fn unload_from_gl(&self);
}

pub trait GLMesh: GLObject {}

pub trait GLTextureArray {
    fn setup_mipmap(&self);
}
