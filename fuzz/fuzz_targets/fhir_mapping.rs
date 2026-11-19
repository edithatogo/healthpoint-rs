#![no_main]

use libfuzzer_sys::fuzz_target;

#[path = "../../crates/healthpoint-fhir/tests/support/fuzz_contract.rs"]
mod contract;

fuzz_target!(|data: &[u8]| {
    contract::check(data);
});
