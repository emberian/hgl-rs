#![crate_type = "rlib"]
#![crate_type = "lib"]

#![crate_id = "github.com/cmr/hgl-rs#hgl:0.0.1"]
#![deny(warnings)]

#![license = "ASL2/MIT"]
#![comment = "Helper utilities for working with OpenGL"]

//! hgl-rs - helpers for working with OpenGL.
//!
//! hgl assumes GL 3.1 with GLSL 140. There are reexports for most types, but
//! constants need to be scoped by the module name.
//!
//! *NOTE*: The various `bind` methods will explicitly bind the object,
//! but the other methods frequently bind themselves too! Be careful what you
//! call if you expect something to be bound to stay bound. They do not
//! restore the current binding before they return.


extern crate gl;

pub use program::{VertexShader, FragmentShader, Shader, Program};
pub use buffer::{Vbo, Ebo, StaticDraw};
pub use query::Query;
pub use vao::Vao;
pub use texture::{ImageInfo, Texture};

use gl::types::{GLuint, GLenum};

pub mod program;
pub mod buffer;
pub mod query;
pub mod vao;
pub mod texture;

/// A simple wrapper for glPrimitiveRestartIndex.
pub fn restart_index(index: GLuint) {
    gl::PrimitiveRestartIndex(index);
}

// move this into container after #12661 and #12660 are fixed

pub enum Primitive {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan
}

impl Primitive {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            Points        => gl::POINTS,
            Lines         => gl::LINES,
            LineStrip     => gl::LINE_STRIP,
            LineLoop      => gl::LINE_LOOP,
            Triangles     => gl::TRIANGLES,
            TriangleStrip => gl::TRIANGLE_STRIP,
            TriangleFan   => gl::TRIANGLE_FAN
        }
    }
}
