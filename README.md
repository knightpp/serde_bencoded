[![crates.io](https://img.shields.io/crates/v/serde_bencoded.svg)](https://crates.io/crates/serde_bencoded)[![Docs](https://docs.rs/serde_bencoded/badge.svg)](https://docs.rs/serde_bencoded/)

# Crate for encoding/decoding bencode

What is bencode? It's the encoding mostly used in `.torrent` files and BitTorrent protocol.
For more info see [BitTorrentSpecification#Bencoding](https://wiki.theory.org/index.php/BitTorrentSpecification#Bencoding).

# Quick example
See `examples` directory
```rust
#[derive(Debug, Serialize, Deserialize)]
struct MetaInfo {
    info: Info,
    announce: String,
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")]
    creation_date: Option<u64>,
    comment: Option<String>,
    #[serde(rename = "created by")]
    created_by: Option<String>,
    encoding: Option<String>,
}

fn main(){
    let string = serde_bencoded::to_string(&MetaInfo{...}).unwrap;
    let mi: MetaInfo = serde_bencoded::from_str(&string).unwrap();
}
```
