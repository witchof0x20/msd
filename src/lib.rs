//! A library for reading and writing MSD files.
//! 
//! # Usage
//! Reading and writing of MSD files is done using [`serde`](https://crates.io/crates/serde), a library
//! for generic serialization and deserialization. It will work with any types that implement the
//! [`Serialize`](https://docs.rs/serde/*/serde/trait.Serialize.html) and
//! [`Deserialize`](https://docs.rs/serde/*/serde/trait.Deserialize.html) traits.
//!
//! # Example
//! ```
//! use std::collections::HashMap;
//!
//! fn main() {
//!     let mut map: HashMap<String, usize> = HashMap::new();
//!     map.insert("foo".to_owned(), 1);
//!     map.insert("bar".to_owned(), 2);
//!
//!     // Serialize the map into a byte buffer.
//!     let serialized = msd::to_bytes(&map).unwrap();
//!
//!     // Deserialize back into a map again.
//!     let deserialized = msd::from_bytes(&serialized).unwrap();
//!
//!     assert_eq!(map, deserialized); 
//! }
//! ```

#![warn(unsafe_op_in_unsafe_fn)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use de::{from_bytes, from_reader, Deserializer};
#[doc(inline)]
pub use ser::{to_bytes, to_writer, Serializer};
