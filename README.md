# test-idioms

A general crate layout for separating generative testing concerns to improve code reuse across different history generators.

In particular, this demonstrates the power of libfuzzer. Two tests execute the same shared test code, defined in the
`shared_correctness_logic` subdirectory, where the default `arbitrary::Arbitrary` and `proptest_derive::Arbitrary` traits are
derived on the shared workload struct (and could be manually defined there, too, if desired).

When running `cargo test` from the root directory, proptest runs with its default configuration and does not find the
bug, which is only triggered on a specific `u64` being input into the system. However, when running cargo fuzz via
`cargo +nightly fuzz run my_fuzz_target` (after installing cargo fuzz with `cargo install cargo-fuzz`) then the
binary instrumentation is able to rapidly converge on the specific `u64` that causes a bug to express.

While this example has a very large test code : production code ratio due to the production code being nonexistent,
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
│   │       ├── crash-aa062a45f55a1402c8b2e0d30fab628dc757b9f3
│   │       └── crash-c39f8b9792fd0e3edd35bd10570ab3fb55e69350
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
