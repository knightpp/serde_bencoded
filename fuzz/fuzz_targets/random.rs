#![no_main]
use libfuzzer_sys::fuzz_target;

mod teststruct;
use teststruct::*;

fuzz_target!(|data: &[u8]| {
    serde_bencoded::from_bytes::<TestStruct>(data).ok();
    // fuzzed code goes here
});
