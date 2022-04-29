#![warn(unsafe_op_in_unsafe_fn)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use de::{Deserializer, from_bytes, from_reader};
#[doc(inline)]
pub use ser::{Serializer, to_bytes, to_writer};
