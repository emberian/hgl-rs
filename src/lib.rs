#[crate_type = "rlib"];
#[crate_type = "lib"];

#[crate_id = "github.com/cmr/hgl-rs#hgl:0.0.1"];
#[deny(warnings)];

#[license = "ASL2/MIT"];
#[comment = "Helper utilities for working with OpenGL"];

//! hgl-rs - helpers for working with OpenGL.
//!
//! hgl assumes GL 3.1 with GLSL 140. It attempts to do complete error
//! checking, and return the information the GL exposes.
//!
//! *NOTE*: The various `activate` methods will explicitly bind the object,
//! but the other methods frequently bind themselves too! Be careful what you
//! call if you expect something to be bound to stay bound. They do not
//! restore the current binding before they return.

extern crate gl;

pub use program::{ShaderType, VertexShader, FragmentShader, Shader, Program};
pub use buffer::{Vbo, Ebo, VboUsage, StaticDraw, DynamicDraw, StreamDraw};
pub use query::{Query, QueryTarget, SamplesPassed};
pub use container::Vao;

use gl::types::{GLuint, GLenum};

mod program;
mod buffer;
mod query;
mod container;

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
