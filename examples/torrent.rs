use serde::{Deserialize, Serialize};
use serde_bencoded::from_bytes;
use serde_bytes::ByteBuf;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = if atty::is(atty::Stream::Stdin) {
        let second_arg = std::env::args_os()
            .nth(1)
            .ok_or("missing `path` argument".to_string())?;
        let mut file = std::fs::File::open(second_arg)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        buf
    } else {
        let mut buf = Vec::new();
        std::io::stdin().lock().read_to_end(&mut buf)?;
        buf
    };
    let info: MetaInfo = from_bytes(&bytes)?;
    println!("announce: {}", info.announce);
    println!("announce-list: {:?}", info.announce_list);
    println!("creation date: {:?}", info.creation_date);
    println!("comment: {:?}", info.comment);
    println!("created by: {:?}", info.created_by);
    println!("encoding: {:?}", info.encoding);

    println!("piece length: {}", info.info.piece_length);
    println!("pieces (length): {}", info.info.pieces.len());
    println!("private: {:?}", info.info.private);
    println!("name: {}", info.info.name);
    // display as Single File Mode
    if let Some(len) = info.info.length {
        println!("\tSingle File Mode");
        println!("\tlength: {}", len);
        println!("\tmd5sum: {:?}", info.info.md5sum);
    }
    // display as Multiple File Mode
    else {
        println!("\tMultiple File Mode");
        println!("\tfiles:");
        for file in info.info.files.unwrap() {
            println!("\t\tlength: {}", file.length);
            println!("\t\tmd5sum: {:?}", file.md5sum);
            println!("\t\tpath: {:?}", file.path);
            println!();
        }
    }

    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
struct Info {
    #[serde(rename = "piece length")]
    piece_length: u64,
    pieces: ByteBuf,
    private: Option<i64>,
    name: String,
    /// if this is `Some` => Single File Mode
    length: Option<u64>,
    md5sum: Option<ByteBuf>,
    /// if this is `Some` => Multiple File mode
    files: Option<Vec<File>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct File {
    length: u64,
    md5sum: Option<ByteBuf>,
    path: Vec<String>,
}

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
