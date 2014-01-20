#[crate_type = "rlib"];
#[crate_type = "lib"];

#[crate_id = "github.com/cmr/hgl-rs#hgl:0.0.1"];

#[license = "ASL2/MIT"];
#[comment = "Helper utilities for working with OpenGL"];

//! hgl-rs - helpers for working with OpenGL.
//!
//! hgl assumes GL 3.1 core profile with GLSL 140. It attempts to do complete
//! error checking, and return the information the GL exposes.

extern mod gl;

use std::unstable::intrinsics::uninit;

/// Shader types
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

impl ShaderType {
    /// Convert a ShaderType into its corresponding GL value
    fn to_glenum(&self) -> gl::GLenum {
        match *self {
            VertexShader => gl::VERTEX_SHADER,
            FragmentShader => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    priv name: gl::GLuint,
    priv type_: ShaderType
}

impl Shader {
    fn new(id: gl::GLuint, type_: ShaderType) -> Shader {
        if cfg!(not(ndebug)) {
            if gl::IsShader(id) == gl::FALSE {
                fail!("id is not a shader!");
            }
        }
        Shader::new_raw(id, type_)
    }

    fn new_raw(id: gl::GLuint, type_: ShaderType) -> Shader {
        Shader { shader: id, type_: type_ }
    }

    /// Returns the name (id) of the shader.
    pub fn name(&self) -> gl::GLuint {
        self.name
    }

    /// Compile a shader.
    ///
    /// Takes the shader contents as a string. On success the Shader is returned.
    /// On failure, the complete log from glGetShaderInfoLog is returned.
    pub fn compile(source: &str, type_: ShaderType) -> Result<Shader, ~str> {
        let gltype = type_.to_glenum();
        let shader = gl::CreateShader(gltype);

        unsafe {
            gl::ShaderSource(shader, 1 as gl::GLsizei, &source.as_ptr() as **gl::GLchar,
                             &source.len() as *gl::GLint);
        }

        let mut ret = gl::FALSE;
        // this is pretty racy if another thread is using gl
        unsafe {
            glGetShaderiv(shader, gl::GL_COMPILE_STATUS, &mut ret as *gl::GLint);
        }

        if ret == gl::FALSE as gl::GLint {
            // oh no, we failed!
            let mut len: gl::GLint = uninit();
            unsafe {
                gl::GetShaderiv(shader, gl::GL_INFO_LOG_LENGTH, &mut len as *gl::GLint);
            }

            // len included trailing null
            let s = std::str::with_capacity(len - 1);

            // XXX: is this string utf8?
            unsafe {
                gl::GetShaderInfoLog(shader, len, &mut len as *gl::GLsizei,
                                     s.as_ptr() as *gl::GLchar);
            }
            s.set_len(len - 1);
            return Err(s);
        }

        Ok(Shader::new_raw(shader, type_))
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::DeleteShader(self.name);
    }
}

/// A program, which consists of multiple compiled shaders "linked" together
pub struct Program {
    name: gl::GLuint
}

impl Program {
    /// Link shaders into a program
    pub fn link(shaders: &[Shader]) -> Result<Program, ~str> {
        let program = gl::CreateProgram();
        for shader in shaders {
            // there are no relevant errors to handle here.
            gl::AttachShader(program, shader.shader);
        }
        gl::LinkProgram(program);

        let mut ret = gl::FALSE;
        // this is pretty racy if another thread is using gl
        unsafe {
            glGetProgramiv(shader, gl::GL_COMPILE_STATUS, &mut ret as *gl::GLint);
        }

        if ret == gl::FALSE as gl::GLint {
            // oh no, we failed!
            let mut len: gl::GLint = uninit();
            unsafe {
                gl::GetProgramiv(shader, gl::GL_INFO_LOG_LENGTH, &mut len as *gl::GLint);
            }

            // len included trailing null
            let s = std::str::with_capacity(len - 1);

            // XXX: is this string utf8?
            unsafe {
                gl::GetProgramInfoLog(shader, len, &mut len as *gl::GLsizei,
                                      s.as_ptr() as *gl::GLchar);
            }
            s.set_len(len - 1);
            return Err(s);
        }


        Ok(Program)
    }
}
