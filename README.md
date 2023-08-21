# test-idioms

An example crate layout for separating generative testing concerns to improve code reuse across different history generators,
invariants, and history execution strategies. It is nice to keep as many testing concerns as possible out of the main business
logic crate.

Additionally, this really demonstrates the power of libfuzzer. Two generative test frameworks execute the same shared actions, invariants and execution strategies defined in the `shared_correctness_logic` subdirectory, where the default `arbitrary::Arbitrary` and `proptest_derive::Arbitrary` traits are
derived on the shared workload struct (and could be manually defined there, too, if desired).

When running `cargo test` from the root directory, proptest runs with its default configuration and does not find the
bug, which is only triggered on a specific `u64` being input into the system. However, when running cargo fuzz via
`cargo +nightly fuzz run my_fuzz_target` (after installing cargo fuzz with `cargo install cargo-fuzz`) then the
binary instrumentation is able to rapidly converge on the specific `u64` that causes a bug to express.

While this example has a very large test code : production code ratio due to the production code being nearly nonexistent,
the test code is mostly boilerplate that does not grow with system complexity, and by decoupling invariants from
execution strategy or workloads, it allows different tests to be written and quickly executed without needing to
add conditional compilation to the production code, or relying on deeply nested `#cfg(test)` inclusion etc...

```
├── Cargo.lock
├── Cargo.toml
├── README.md
├── fuzz
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── artifacts
│   │   └── my_fuzz_target
│   │       ├── (found crashes accumulate here)
│   ├── corpus
│   │   └── my_fuzz_target
│   │       └── (fuzzing corpus accumulates here)
│   └── fuzz_targets
│       └── my_fuzz_target.rs
├── shared_correctness_logic
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── src
│   ├── bin
│   │   └── my_binary.rs
│   └── lib.rs
└── tests
    └── proptest.rs
```
