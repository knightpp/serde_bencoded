#![no_main]
use libfuzzer_sys::{arbitrary::{Unstructured, Arbitrary}, fuzz_target};
use serde_bencoded::{from_str, from_str_auto, to_string};

mod teststruct;
use teststruct::*;

fuzz_target!(|data: &[u8]| {
    let mut unstructured = Unstructured::new(data);
    if let Ok(ts) = TestStruct::arbitrary(&mut unstructured) {
        let ts_decoded: TestStruct = from_str(
            &to_string(&ts)
                .unwrap_or_else(|err| panic!("\nError: {}\nInput data: {:#?}", err, &ts)),
        )
        .unwrap_or_else(|err| panic!("\nError: {:?}\nInput data: {:#?}", err, &ts));
        assert_eq!(ts, ts_decoded);
    }
    if let Ok(ts) = TestStructAuto::arbitrary(&mut unstructured) {
        let ts_decoded: TestStructAuto = from_str_auto(
            &to_string(&ts)
                .unwrap_or_else(|err| panic!("\nError: {}\nInput data: {:#?}", err, &ts)),
        )
        .unwrap_or_else(|err| panic!("\nError: {:?}\nInput data: {:#?}", err, &ts));
        assert_eq!(ts, ts_decoded);
    }
});
