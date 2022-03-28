#![no_std]

extern crate alloc;

mod de;
mod error;
mod ser;

pub use de::Deserializer;
pub use error::{Error, Result};
pub use ser::Serializer;
