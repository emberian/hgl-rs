use gl;
use std;
use std::libc::c_void;
use gl::types::{GLuint, GLenum, GLsizeiptr};

/// A vertex buffer object
pub struct Vbo {
    name: GLuint
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.name); }
    }
}

/// Frequency with which the vbo is expected to be updated
pub enum VboUsage {
    /// Updated once, drawn many times
    StaticDraw,
    /// Updated many times, drawn many times
    DynamicDraw,
    /// Updated once, drawn once
    StreamDraw
    // TODO: add Read and Copy variants
}

impl VboUsage {
    fn to_glenum(&self) -> GLenum {
        match *self {
            StaticDraw  => gl::STATIC_DRAW,
            DynamicDraw => gl::DYNAMIC_DRAW,
            StreamDraw  => gl::STREAM_DRAW,
        }
    }
}

impl Vbo {
    /// Generate a new VBO and upload `data` to it.
    pub fn from_data<T>(data: &[T], usage: VboUsage) -> Result<Vbo, ~str> {
        let mut vbo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo as *mut GLuint); }
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        unsafe {
            info!("{} elements at {}", data.len() * std::mem::size_of::<T>(), data.as_ptr() as uint);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                           data.as_ptr() as *c_void, usage.to_glenum());
        }
        // TODO: check BufferData error
        Ok(Vbo { name: vbo })
    }

    pub fn activate(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.name);
    }
}

/// An Element Buffer Object, aka GL_ELEMENT_ARRAY_BUFFER.
pub struct Ebo {
    name: GLuint
}

impl Ebo {
    /// Create an EBO from a slice of indices
    pub fn from_indices(indices: &[GLuint]) -> Ebo {
        let mut ebo = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        unsafe { gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                                indices.as_ptr() as *c_void, gl::STATIC_DRAW);
        }
        Ebo { name: ebo }
    }

    pub fn activate(&self) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.name);
    }
}

