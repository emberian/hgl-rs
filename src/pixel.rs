//! Enums for pixel format and pixel types, for pixel transfers (particularly in textures).
#![allow(non_camel_case_types)]

use gl;
use gl::types::GLenum;


pub enum PixelFormat {
    RED,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,
}

impl PixelFormat {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            RED => gl::RED,
            RG => gl::RG,
            RGB => gl::RGB,
            BGR => gl::BGR,
            RGBA => gl::RGBA,
            BGRA => gl::BGRA,
        }
    }
}

pub enum PixelType {
    UNSIGNED_BYTE,
    BYTE,
    UNSIGNED_SHORT,
    SHORT,
    UNSIGNED_INT,
    INT,
    FLOAT,
    UNSIGNED_BYTE_3_3_2,
    UNSIGNED_BYTE_2_3_3_REV,
    UNSIGNED_SHORT_5_6_5,
    UNSIGNED_SHORT_5_6_5_REV,
    UNSIGNED_SHORT_4_4_4_4,
    UNSIGNED_SHORT_4_4_4_4_REV,
    UNSIGNED_SHORT_5_5_5_1,
    UNSIGNED_SHORT_1_5_5_5_REV,
    UNSIGNED_INT_8_8_8_8,
    UNSIGNED_INT_8_8_8_8_REV,
    UNSIGNED_INT_10_10_10_2,
    UNSIGNED_INT_2_10_10_10_REV,
}

impl PixelType {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            UNSIGNED_BYTE               => gl::UNSIGNED_BYTE,
            BYTE                        => gl::BYTE,
            UNSIGNED_SHORT              => gl::UNSIGNED_SHORT,
            SHORT                       => gl::SHORT,
            UNSIGNED_INT                => gl::UNSIGNED_INT,
            INT                         => gl::INT,
            FLOAT                       => gl::FLOAT,
            UNSIGNED_BYTE_3_3_2         => gl::UNSIGNED_BYTE_3_3_2,
            UNSIGNED_BYTE_2_3_3_REV     => gl::UNSIGNED_BYTE_2_3_3_REV,
            UNSIGNED_SHORT_5_6_5        => gl::UNSIGNED_SHORT_5_6_5,
            UNSIGNED_SHORT_5_6_5_REV    => gl::UNSIGNED_SHORT_5_6_5_REV,
            UNSIGNED_SHORT_4_4_4_4      => gl::UNSIGNED_SHORT_4_4_4_4,
            UNSIGNED_SHORT_4_4_4_4_REV  => gl::UNSIGNED_SHORT_4_4_4_4_REV,
            UNSIGNED_SHORT_5_5_5_1      => gl::UNSIGNED_SHORT_5_5_5_1,
            UNSIGNED_SHORT_1_5_5_5_REV  => gl::UNSIGNED_SHORT_1_5_5_5_REV,
            UNSIGNED_INT_8_8_8_8        => gl::UNSIGNED_INT_8_8_8_8,
            UNSIGNED_INT_8_8_8_8_REV    => gl::UNSIGNED_INT_8_8_8_8_REV,
            UNSIGNED_INT_10_10_10_2     => gl::UNSIGNED_INT_10_10_10_2,
            UNSIGNED_INT_2_10_10_10_REV => gl::UNSIGNED_INT_2_10_10_10_REV,
        }
    }
}
