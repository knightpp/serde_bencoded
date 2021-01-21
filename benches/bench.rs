use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
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

fn criterion_benchmark(c: &mut Criterion) {
    let torrent = MetaInfo {
        announce: "http://127.0.0.1:8080/announce".to_owned(),
        announce_list: None,
        creation_date: Some(1611237267),
        comment: Some("My test comment".to_owned()),
        created_by: Some("Transmission/3.00 (bb6b5a062e)".to_owned()),
        encoding: Some("UTF-8".to_owned()),
        info: Info {
            piece_length: 32768,
            pieces: ByteBuf::from(vec![
                93, 158, 15, 80, 241, 36, 93, 55, 158, 152, 158, 8, 213, 3, 254, 220, 139, 73, 2,
                102, 222, 167, 1, 80, 63, 196, 73, 80, 58, 110, 133, 130, 42, 39, 174, 191, 2, 153,
                70, 62,
            ]),
            private: Some(1),
            name: "src".to_owned(),
            length: None,
            md5sum: None,
            files: Some(vec![
                File {
                    length: 20577,
                    md5sum: None,
                    path: vec!["de.rs".to_owned()],
                },
                File {
                    length: 3702,
                    md5sum: None,
                    path: vec!["error.rs".to_owned()],
                },
                File {
                    length: 3901,
                    md5sum: None,
                    path: vec!["lib.rs".to_owned()],
                },
                File {
                    length: 19776,
                    md5sum: None,
                    path: vec!["ser.rs".to_owned()],
                },
                File {
                    length: 7862,
                    md5sum: None,
                    path: vec!["ser".to_owned(), "only_string_ser.rs".to_owned()],
                },
            ]),
        },
    };
    c.bench_function("serialize", |b| {
        b.iter(|| serde_bencoded::to_string(black_box(&torrent)))
    });
}

fn criterion_benchmark2(c: &mut Criterion) {
    const DATA: &[u8] = include_bytes!("./bench.torrent");
    c.bench_function("deserialize", |b| {
        b.iter(|| serde_bencoded::from_bytes::<MetaInfo>(DATA))
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark2);
criterion_main!(benches);
