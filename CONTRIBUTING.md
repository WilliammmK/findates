# Contributing to findates

Contributions are welcome — bug reports, API feedback, new conventions,
and calendar data are all useful at this stage.

## Getting started

```bash
git clone https://github.com/WilliammmK/findates
cd findates
cargo test --all-features
```

All tests should pass before and after any change.

## What is useful right now

**Bug reports and API feedback** — if something behaves unexpectedly or
the API is awkward to use in a real financial context, open an issue.
Domain expertise from practitioners is particularly valuable.

**Day count conventions** — the roadmap includes `ActActICMA`, `Act365Fixed`,
`Thirty360 US`, and others. If you've implemented one of these before
or know the specification well, a PR with implementation and tests would be
a great contribution. If you're unsure whether an implementation is correct,
open an issue first and we can work through it together.

**Calendar data** — a companion crate `findates-calendars` with maintained
holiday calendars for major financial centres is planned. If you have domain
knowledge of a specific market's holiday rules, get in touch via an issue.

**Documentation improvements** — corrections, clearer examples, and better
explanations of financial concepts for developers who aren't domain experts
are always welcome.

## Guidelines

- Keep changes focused — one concern per pull request
- All public items must have doc comments with at least one example
- New conventions must include tests validated against known financial values,
  not just round-trip or smoke tests
- Follow idiomatic Rust — prefer pure functions, avoid unnecessary state,
  use `Result` and `Option` rather than panicking
- Run `cargo clippy --all-features -- -D warnings` and
  `cargo doc --all-features --no-deps` before submitting — both should
  be warning-free

## Running the full check suite

```bash
cargo test --all-features
cargo test --no-default-features
cargo clippy --all-features -- -D warnings
cargo doc --all-features --no-deps
```

## License

By contributing you agree that your contributions will be licensed under
the same MIT OR Apache-2.0 terms as the rest of the project.
