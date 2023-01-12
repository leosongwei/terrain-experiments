use gl;
use glam::{Mat4, Vec2, Vec3};
use std;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::str::FromStr;

pub struct ShaderProgram {
    id: gl::types::GLuint,
    uniform_locations: HashMap<String, i32>,
}

impl std::ops::Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub enum ShaderParam {
    Int(i32),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Mat4(Mat4),
}

impl ShaderProgram {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_uniforms(&self, params: HashMap<&str, ShaderParam>) {
        let set_uniform = |name: &str, value: ShaderParam| {
            let uniform_location = self
                .uniform_locations
                .get(name)
                .expect("Shader uniform not found!")
                .clone();
            unsafe {
                match value {
                    ShaderParam::Int(v) => gl::Uniform1i(uniform_location, v),
                    ShaderParam::Float(v) => gl::Uniform1f(uniform_location, v),
                    ShaderParam::Vec2(v) => gl::Uniform2f(uniform_location, v.x, v.y),
                    ShaderParam::Vec3(v) => gl::Uniform3f(uniform_location, v.x, v.y, v.z),
                    ShaderParam::Mat4(v) => {
                        let ptr = v.to_cols_array().as_ptr();
                        gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, ptr);
                    }
                }
            }
        };
        for (name, value) in params {
            set_uniform(name, value);
        }
    }

    pub fn from_shader_strings(
        vs: &str,
        fs: &str,
        uniforms: Vec<&str>,
    ) -> Result<ShaderProgram, String> {
        let program_id = unsafe { gl::CreateProgram() };

        let vs = match shader_from_source(vs, gl::VERTEX_SHADER) {
            Ok(id) => id,
            Err(message) => return Err(String::from("VERTEX_SHADER:") + &message),
        };
        let fs = match shader_from_source(fs, gl::FRAGMENT_SHADER) {
            Ok(id) => id,
            Err(message) => return Err(String::from("FRAGMENT_SHADER:") + &message),
        };

        unsafe {
            gl::AttachShader(program_id, vs.id);
            gl::AttachShader(program_id, fs.id);
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            unsafe {
                gl::DeleteProgram(program_id);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        let mut uniform_locations: HashMap<String, i32> = HashMap::new();

        unsafe {
            for uniform in uniforms {
                uniform_locations.insert(
                    String::from_str(uniform).unwrap(),
                    gl::GetUniformLocation(program_id, uniform.as_ptr() as *const i8),
                );
            }
        }

        unsafe {
            gl::DetachShader(program_id, vs.id);
            gl::DetachShader(program_id, fs.id);
        }

        Ok(ShaderProgram {
            id: program_id,
            uniform_locations,
        })
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl std::ops::Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

fn shader_from_source(source: &str, kind: gl::types::GLenum) -> Result<Shader, String> {
    let c_source = CString::new(source).unwrap();
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &c_source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(Shader { id })
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
