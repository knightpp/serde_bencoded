use libfuzzer_sys::arbitrary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, arbitrary::Arbitrary)]
enum TestEnum {
    A,
    B(()),
    C(String),
    D(Vec<u64>),
    E(std::collections::HashMap<String, String>),
    F((u64, u64)),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, arbitrary::Arbitrary)]
struct UnitStruct;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, arbitrary::Arbitrary)]
struct NewtypeStruct(u8);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, arbitrary::Arbitrary)]
#[serde(tag = "t", content = "c")]
enum E {
    N(u8),
    Z(u16),
    X(u32),
    U(u64),
}
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, arbitrary::Arbitrary)]
#[serde(tag = "y")]
enum K {
    E(E),
    F(E),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, arbitrary::Arbitrary)]
pub struct TestStruct {
    test_enum: TestEnum,
    unit: (),
    str: String,
    u64: u64,
    char: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    option: Option<i64>,
    i64: i64,
    unit_struct: UnitStruct,
    newtype_struct: NewtypeStruct,
    tuple: (u64, String),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, arbitrary::Arbitrary)]
pub struct TestStructAuto {
    nested_enum_adjacently_tagged: K,
}
