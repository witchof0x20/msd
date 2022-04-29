#![warn(unsafe_op_in_unsafe_fn)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use de::{from_bytes, from_reader, Deserializer};
#[doc(inline)]
pub use ser::{to_bytes, to_writer, Serializer};
