# findates

[![Crates.io](https://img.shields.io/crates/v/findates.svg)](https://crates.io/crates/findates)
[![Docs.rs](https://docs.rs/findates/badge.svg)](https://docs.rs/findates)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![MSRV: 1.70](https://img.shields.io/badge/rustc-1.70+-blue.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

Financial date arithmetic for Rust.

`findates` handles the unglamorous but essential layer that every financial
application depends on: business day calendars, date adjustment conventions,
day count fractions, and schedule generation. It is designed to be lightweight,
composable, and independent of larger quantitative finance frameworks.

## Installation

```toml
[dependencies]
findates = "0.1"
```

Optional serde support:

```toml
findates = { version = "0.1", features = ["serde"] }
```

## Usage

```rust
use chrono::NaiveDate;
use findates::calendar::basic_calendar;
use findates::conventions::{AdjustRule, DayCount, Frequency};
use findates::schedule::Schedule;
use findates::algebra;

// Adjust a Saturday to the next business day (Monday)
let cal = basic_calendar();
let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
let monday = algebra::adjust(&saturday, Some(&cal), Some(AdjustRule::Following));

// Generate a semi-annual schedule
let anchor = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
let end    = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let dates  = Schedule::new(Frequency::Semiannual, None, None)
    .generate(&anchor, &end)
    .unwrap();
// [2023-01-01, 2023-07-01, 2024-01-01]

// Compute a day count fraction
let dcf = algebra::day_count_fraction(
    &anchor, &end, DayCount::Act365, Some(&cal), Some(AdjustRule::Following),
).unwrap();
```

## Features

- Business day calendars with weekend and holiday sets, union and intersection
- Standard financial adjustment rules for non-business dates
- Day count fraction calculations covering the most common market conventions
- Frequency-based schedule generation with lazy infinite iterators
- Optional `serde` support for all types

For the full list of supported conventions, adjustment rules, and frequencies
see the [documentation](https://docs.rs/findates).

## Design

Core operations are pure functions — no hidden state, no side effects.
Schedules are lazy iterators; dates are only computed when needed. All date
types are `chrono::NaiveDate`. Timezone-aware dates are out of scope —
financial date arithmetic operates on calendar dates without reference to
time of day.

## Dependencies

`findates` uses [`chrono`](https://docs.rs/chrono) as its date representation.
All public functions accept and return `chrono::NaiveDate`. If your codebase
uses the [`time`](https://docs.rs/time) crate, conversion at the boundary is
currently required. Broader date type interoperability is planned for a future
release.

## Non-goals

`findates` is not a pricing library, a risk engine, or a Rust replacement for
QuantLib. It is a foundation layer — the date arithmetic that everything
else sits on top of.

## License

Licensed under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE) at your option.