pub trait GLObject {
    fn is_loaded_in_gl(&self) -> bool;
    fn load_into_gl(&self);
    fn unload_from_gl(&self);
}

pub trait GLMesh: GLObject {}

pub trait GLTexture: GLObject {
    fn setup_mipmap(&self);
}
