use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Arbitrary)]
enum TestEnum {
    A,
    B(()),
    C(String),
    D(Vec<u64>),
    E(std::collections::HashMap<String, String>),
    F((u64, u64)),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Arbitrary)]
struct UnitStruct;
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Arbitrary)]
struct NewtypeStruct(u8);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Arbitrary)]
#[serde(tag = "t", content = "c")]
enum E {
    N(u8),
}
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Arbitrary)]
#[serde(tag = "y")]
enum K {
    E(E),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Arbitrary)]
pub struct TestStruct {
    test_enum: TestEnum,
    unit: (),
    str: String,
    u64: u64,
    char: char,
    option: Option<i64>,
    i64: i64,
    unit_struct: UnitStruct,
    newtype_struct: NewtypeStruct,
    tuple: (u64, String),
    nested_enum_adjacently_tagged: K,
}
