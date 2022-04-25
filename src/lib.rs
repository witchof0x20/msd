#![warn(unsafe_op_in_unsafe_fn)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use de::Deserializer;
#[doc(inline)]
pub use ser::Serializer;
