use proptest::prelude::*;

use shared_correctness_logic::{execute_and_check_invariants, MyInvariant, Workload};

proptest! {
  #[test]
  fn test_workload(workload: Workload) {
    let invariants = vec![Box::new(MyInvariant::default()) as _];
    execute_and_check_invariants(workload, invariants);
  }
}
