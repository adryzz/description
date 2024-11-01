#![no_std]

pub use description_macro::*;
/// Like [`Display`], but [`'static`], no_std and no_alloc.
pub trait Description {
    fn description(&self) -> &'static str;
}

/// Like [`Display`], but [`'static`], no_std and no_alloc.
pub trait OptionalDescription {
    fn description(&self) -> Option<&'static str>;
}