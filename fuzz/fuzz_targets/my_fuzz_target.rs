#![no_main]

use libfuzzer_sys::fuzz_target;

use shared_correctness_logic::{execute_and_check_invariants, MyInvariant, Workload};

fuzz_target!(|workload: Workload| {
    let invariants = vec![Box::new(MyInvariant::default()) as _];
    execute_and_check_invariants(workload, invariants);
});
