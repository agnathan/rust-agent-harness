# How the numbers were measured

A table in a chapter reports the mean. Criterion's printed `time:` line is the slope of its regression, not that mean. The mean is `mean.point_estimate` in the run's `estimates.json`. The `change` line is a comparison with a previous baseline on that machine.

The flags are `--warm-up-time 1 --measurement-time 2 --sample-size 10`, placed after `--` so Cargo passes them to Criterion:

```bash
cargo bench -p <package> --bench <name> --locked -- --warm-up-time 1 --measurement-time 2 --sample-size 10
```

The package and the bench name are the ones in that chapter. `.github/scripts/run_bench_section.sh` is what the Benchmarks workflow runs for one section.

Tables in chapters 1 through 8, and the local timings quoted beside an early performance box, were measured on a 4-vCPU Intel Xeon guest under KVM, with 16 GiB of RAM, Linux, rustc 1.99.0, and cargo-nextest 0.9.148. From chapter 9 on, a mean cited with a run link is `mean.point_estimate` from the Benchmarks workflow on `ubuntu-latest`. Several chapters can share one run when they were measured together. The section number in the chapter is the bench section, and it matches the chapter number.

An older id in `benchmarks/descriptions.json` is an alias left from a chapter renumber. The function that was timed did not change when the id did.

CI runs the tests on Ubuntu, macOS, and Windows. The benches are the Ubuntu job. A push to `main` appends the medians to the [benchmark dashboard](https://agnathan.github.io/rust-agent-harness/). The benches time the step crate, not `my-harness/`.
