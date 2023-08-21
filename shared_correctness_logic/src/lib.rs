use my_crate::MyObject;

// Required code follows:

/// If the default generators are inappropriate, then this is where they would
/// be overridden, as this is where we can implement those traits for the Action.
#[derive(Debug, arbitrary::Arbitrary, proptest_derive::Arbitrary)]
pub enum Action {
    /// any stateful operations on the system under test can be represented
    /// here.
    Apply(u64),
}

#[derive(Debug, arbitrary::Arbitrary, proptest_derive::Arbitrary)]
pub struct Workload {
    actions: Vec<Action>,
}

// Potentially nice-to-have shared code, maybe in a shared testing crate:

/// This can be more generic if we want to do something reusable in shared test code.
#[allow(unused)]
pub trait StatefulInvariant {
    fn precondition(&mut self, my_object: &MyObject, action: &Action) -> bool {
        true
    }

    fn postcondition(&mut self, my_object: &MyObject, action: &Action) -> bool {
        true
    }
}

#[derive(Default)]
pub struct MyInvariant {
    sum_of_inputs: u64,
}

/// Only implement the things that are interesting
impl StatefulInvariant for MyInvariant {
    fn postcondition(&mut self, my_object: &MyObject, action: &Action) -> bool {
        my_object.get_state() != 0x8765_2345
    }
}

/// While in some cases it may be nice to use proptest_state_machine,
/// this lets us work with the same general strategy from multiple
/// test crates that could be good at generating different kinds of
/// histories - especially cargo fuzz, fuzzcheck etc...
pub fn execute_and_check_invariants(
    workload: Workload,
    mut invariants: Vec<Box<dyn StatefulInvariant>>,
) -> bool {
    let mut system_under_test = MyObject::new();

    for action in workload.actions {
        for invariant in &mut invariants {
            if !invariant.precondition(&system_under_test, &action) {
                return false;
            };
        }

        match &action {
            Action::Apply(input) => system_under_test.apply(*input),
        }

        for invariant in &mut invariants {
            if !invariant.postcondition(&system_under_test, &action) {
                return false;
            };
        }
    }

    true
}
