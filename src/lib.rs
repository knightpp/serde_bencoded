/*!
    ## Caveats
    ### Serializer
    `serde` treats `[u8; 1-32]`, `Vec<u8>`, `&[u8]` like any other
    sequence, that is it will be encoded as list. Sollution - use
    [`serde_bytes`](https://github.com/serde-rs/bytes)

    The crate doesn't sort by keys in bencoded dictionaries.
    That is, the order of elements depends on hashing algorithm
    of `HashMap` or on the order of fields of a `struct`.
    TODO: fix me?

    ## Alternatives
    Arbitrary order. Search more on [`crates.io`](https://crates.io/search?q=bencode) or
    [`lib.rs`](https://lib.rs/search?q=bencode).

    - [`bendy`](https://lib.rs/crates/bendy)
    - [`bencode`](https://lib.rs/crates/bencode)
    - [`serde_bencode`](https://lib.rs/crates/serde_bencode)
    - [`bt_bencode`](https://lib.rs/crates/bt_bencode)
    - [`bip_bencode`](https://lib.rs/crates/bip_bencode)
    

    ## Bencoding
    ### Supported data types
    - [`Byte Strings`](#byte-strings)
    - [`Integers`](#integers)
    - [`Lists`](#lists)
    - [`Dictionaries`](#dictionaries)
    #### Byte Strings
    `<base ten ASCII>:<string bytes>`

    Examples:
    - `4:abcd`
    - `0:`

    #### Integers
    `i<base ten ASCII, optional minus sign>e`

    The maximum number is not specified. This crate
    handles integers as [`u64`](u64).

    Examples:
    - `i123456e`
    - `i-5e`

    Malformed:
    - `i03e`, anything that starts with 0, except `i0e`
    - `i-0e`

    #### Lists
    `l<bencode type values>e`

    Examples:
    - `li0ei1ei2ee` == `[1,2,3]`
    - `le` == `[]`

    #### Dictionaries
    `d<bencoded string><bencoded element>e`

    Keys must be sorted as __raw__ strings. [`string`](#byte-strings)'s should be
    compared using a __binary comparison__.

    Examples:
    - `de` == `{}`
    - `d4:rustl2:is7:awesomeee` == `{"rust" => ["is", "awesome"]}`

    ## Crate features
    ### Bool
    Enables serializing/deserializing of [`bool`](bool). It's stored as
    [`integer`](#integers). `0` for `false` and `1` for `true`.
*/

mod de;
mod error;
mod ser;

// pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, to_writer, Serializer};
