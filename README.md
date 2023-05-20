# msd

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Anders429/msd/test.yml?branch=master)](https://github.com/Anders429/msd/actions)
[![codecov.io](https://img.shields.io/codecov/c/gh/Anders429/msd)](https://codecov.io/gh/Anders429/msd)
[![crates.io](https://img.shields.io/crates/v/msd)](https://crates.io/crates/msd)
[![docs.rs](https://docs.rs/msd/badge.svg)](https://docs.rs/msd)
[![MSRV](https://img.shields.io/badge/rustc-1.58.0+-yellow.svg)](#minimum-supported-rust-version)
[![License](https://img.shields.io/crates/l/msd)](#license)

A library for reading and writing MSD files.

## Usage
Reading and writing of MSD files is done using [`serde`](https://crates.io/crates/serde), a library
for generic serialization and deserialization. It will work with any types that implement the
[`Serialize`](https://docs.rs/serde/*/serde/trait.Serialize.html) and
[`Deserialize`](https://docs.rs/serde/*/serde/trait.Deserialize.html) traits.

### Example
``` rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, usize> = HashMap::new();
    map.insert("foo".to_owned(), 1);
    map.insert("bar".to_owned(), 2);

    // Serialize the map into a byte buffer.
    let serialized = msd::to_bytes(&map).unwrap();

    // Deserialize back into a map again.
    let deserialized = msd::from_bytes(&serialized).unwrap();

    assert_eq!(map, deserialized); 
}
```

### Unrepresentable Types
Not all [`serde` types](https://serde.rs/data-model.html#types) can be represented in MSD format.
Some compound types cannot be encoded due to the ambiguity that would arise when decoding them.
Specifically, these unrepresentable `serde` types are:

- `tuple` or `tuple_struct` containing `option`.
- `tuple` or `tuple_struct` containing `seq`.
- `tuple` or `tuple_struct` containing `map`.
- `tuple` or `tuple_struct` containing `struct`.
- `struct` containing another `struct` as a field value.
- `seq` containing `option`.
- `seq` containing another `seq` as an element.

Additionally, this library cannot deserialize types that are intended to be deserialized as
self-describing, and both `struct` fields and `enum` variants must be deserialized as identifiers.
See `serde`'s
[`Deserializer` documentation](https://docs.rs/serde/latest/serde/de/trait.Deserializer.html) for
more details.

## About MSD Files
MSD is a configuration file format that has been in use since the late 90s. It has mainly seen
usage in rhythm dance games such as [*Stepmania*](https://github.com/stepmania/stepmania) and
[*Dancing With Intensity*](http://dwi.ddruk.com/), but it is suitable to be used in many other
contexts. MSD files benefit from being human-readable and easily modifiable.

### Format
While the MSD file format has never been formally specified, the format has been informally defined
as any number of tags paired with an optional number of parameters in the following format:
`#TAG:PARAM0:PARAM1:PARAM2;`. Tags and parameters can be any sequence of bytes, with the
requirement that `:`, `;`, `#`, and `\` bytes are escaped.

Comments may be made with a leading `//`. Any pair of `//` that are not intended to create a
comment should be escaped. Note that while this library can parse MSD files with comments, it
cannot write comments.

Some informal usages of MSD have allowed multiple parameter lists for a single tag, most notably
Dancing With Intensity's `.dwi` files (which is written with MSD format) with its
[`#BACKGROUND` tag](http://dwi.ddruk.com/readme.php#background). This library is equipped to read
and write these types of tags as well, interpreting them as the `serde` map data type.

### Origin
The first known use of the MSD format is from the rhythm dance game DDR'99 (DDR here standing for
"Delight Delight Reduplication"). The earliest known specification of the `.msd` files used is
[version 2.0β1](https://web.archive.org/web/20070606025338/http://doremi.kalin.to/ddr/msd_format.html).
Note that `.msd` files are not necessarily the same as what is commonly referred to as an "MSD
File"; `.msd` files often contain specific tags, whereas an "MSD file" is any file that follows the
general MSD tag-parameter structure (See also Stepmania's definition of an
[MSD file](https://github.com/stepmania/stepmania/blob/a1984e4f6519b1376ac030d8c6b11c3aff1dcae6/src/MsdFile.cpp#L2-L7)).

As for what MSD originally stood for, no one seems to know for sure. The best guess is likely
"Musical Score Data", but there isn't a concrete source to back this up. Modern usages of the
format simply refer to it as "MSD".

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.58.0` and up.

## License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/substring/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/substring/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
