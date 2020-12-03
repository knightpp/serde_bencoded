use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, Serialize, Deserialize)]
struct UnitStruct;
#[derive(Debug, Serialize, Deserialize)]

enum TestEnum {
    A,
    B,
    C,
}

#[derive(Debug, Serialize, Deserialize)]

struct TestStruct {
    string: String,
    tuple: (u64, u8),
    bytes: ByteBuf,
    unit: (),
    unit_struct: UnitStruct,
    test_enum: TestEnum,
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("serialize", |b| {
        b.iter(|| {
            serde_bencoded::to_string(black_box(&TestStruct {
                string: "hello world".to_string(),
                tuple: (6023124, 0xFF),
                bytes: ByteBuf::from(vec![1, 2, 3]),
                unit: (),
                unit_struct: UnitStruct,
                test_enum: TestEnum::A,
            }))
        })
    });
}

fn criterion_benchmark2(c: &mut Criterion) {
    const DATA: &[u8] = &[
        100, 49, 49, 58, 117, 110, 105, 116, 95, 115, 116, 114, 117, 99, 116, 49, 48, 58, 85, 110,
        105, 116, 83, 116, 114, 117, 99, 116, 52, 58, 117, 110, 105, 116, 48, 58, 53, 58, 98, 121,
        116, 101, 115, 51, 58, 1, 2, 3, 53, 58, 116, 117, 112, 108, 101, 108, 105, 54, 48, 50, 51,
        49, 50, 52, 101, 105, 50, 53, 53, 101, 101, 54, 58, 115, 116, 114, 105, 110, 103, 49, 49,
        58, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 57, 58, 116, 101, 115, 116, 95,
        101, 110, 117, 109, 49, 58, 65, 101,
    ];
    c.bench_function("deserialize", |b| {
        b.iter(|| serde_bencoded::from_bytes::<TestStruct>(DATA))
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark2);
criterion_main!(benches);
