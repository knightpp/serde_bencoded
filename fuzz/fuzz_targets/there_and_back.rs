#![no_main]
use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use serde_bencoded::{from_str, to_string};

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
});
