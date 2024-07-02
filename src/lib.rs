pub mod window;
pub mod gl_utils;
pub mod renderers;

pub use gl;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
