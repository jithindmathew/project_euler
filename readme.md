# Solutions to first 100 problems in [Project Euler](https://projecteuler.net/archives) in Rust

#### Solve problem

Solve a single problem

```
cargo run -- 3
```

output:

```
===========================================================
Problem : 3
-----------------------------------------------------------
Answer : 6857
solution_1 took 2.8894ms
-----------------------------------------------------------
Answer : 6857
solution_2 took 55.2687ms
===========================================================
```

---

Solve multiple problems

```
cargo run -- 3 7 12
```

output:

```
===========================================================
Problem : 3
-----------------------------------------------------------
Answer : 6857
solution_1 took 3.4676ms
-----------------------------------------------------------
Answer : 6857
solution_2 took 52.8961ms
===========================================================
Problem : 7
-----------------------------------------------------------
Answer : 104743
solution_1 took 374.7079ms
===========================================================
Problem : 12
-----------------------------------------------------------
Answer : 76576500
solution_1 took 420.0793ms
===========================================================
```

---

#### Create and open documentation

```
cargo doc --no-deps --open
```

`cargo doc --open` will create docs for dependencies too, which takes a lot of space and is unnecessary.

Documentation is created in `target/doc/project_euler/`. Open `index.html` to open manually.

Create/edit documentation by adding/editing documentation comments if required.

references : [cargo-doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)

---

#### Run Doctest

```
cargo test --doc
```

tests the examples given in documentation comments.

Note: _There are no other tests other than doctests and_ `cargo test` _runs all doctests._

references : [Documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)

---

#### Run Benchmarks

Run all benchmarks

```
cargo bench
```

Run a single benchmark
```
cargo bench -- <name>
```

Refer `benches/benchmarks.rs` for more info on benchmark parameters.

Benchmarking report is available at `target/criterion/report/`. Open `index.html` to view report.

References : [cargo-bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)

---

#### Why only 100 problems?

Project Euler gives permission only to publish solutions to first 100 problems.

---

#### Are these the fastest/most-efficient solutions?

No, but good enough.

---
