# Findates

[![Crates.io](https://img.shields.io/crates/v/findates.svg)](https://crates.io/crates/findates)
[![Docs.rs](https://docs.rs/findates/badge.svg)](https://docs.rs/findates)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![MSRV: 1.70](https://img.shields.io/badge/rustc-1.70+-blue.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

**Findates** is a Rust library for handling dates in financial applications — including schedules, business day adjustments, and day count conventions.

Any meaningful financial calculation relies on a notion of time. While there is extensive literature on pricing models and financial theory, much less attention is given to the practical task of constructing the time inputs those models depend on.

Findates focuses on this layer:
> generating correct schedules, applying conventions, and computing time/dates consistently.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
findates = "0.1"
```

---

## Motivation

Financial applications rely heavily on accurate and consistent handling of time:

- business day calculations  
- holiday calendars  
- day count conventions  
- schedule generation  

Despite their importance, these are often:
- reimplemented repeatedly  
- tightly coupled to larger systems  
- difficult to extend or maintain  

Findates was developed to provide:
- a **lightweight and focused alternative**
- a **clear and modular structure**
- an implementation aligned with **Rust's strengths** (type safety, iterators, composability)

---

## Design Approach

The library follows a deliberately simple structure, separating the main concepts involved in financial date handling:

- `calendar`: definition of working days (weekends and holidays)
- `conventions`: financial rules (adjustment rules, day count conventions, frequencies)
- `algebra`: core operations on dates
- `schedule`: generation of date sequences based on frequency

A more **functional approach** is taken where appropriate:
- core operations are implemented as standalone functions
- no hidden state or side effects
- behavior is explicit through inputs (calendar, rules, conventions)

At the same time, the design avoids unnecessary abstraction — the goal is clarity and practical usability rather than completeness or generality.

---

## Key Features

- Business day determination based on custom calendars  
- Date adjustment using standard financial conventions (Following, ModFollowing, Preceding, Nearest, …)
- Frequency-based schedule generation with lazy iterators
- Day count fraction calculations (Act/360, Act/365, Act/Act ISDA, 30/360 Euro, 30/365, BD/252)
- Calendar union and intersection for multi-jurisdiction trades

---

## Example

```rust
use chrono::NaiveDate;
use findates::calendar::basic_calendar;
use findates::conventions::{AdjustRule, DayCount, Frequency};
use findates::schedule::Schedule;
use findates::algebra;

// Build a calendar with Sat/Sun as weekend
let cal = basic_calendar();

// Adjust a Saturday to the next business day
let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
let adj = algebra::adjust(&saturday, Some(&cal), Some(AdjustRule::Following));
assert_eq!(adj, NaiveDate::from_ymd_opt(2024, 3, 18).unwrap()); // Monday

// Generate a semi-annual schedule
let anchor = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
let end    = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let sched  = Schedule::new(Frequency::Semiannual, None, None);
let dates  = sched.generate(&anchor, &end).unwrap();
// dates: [2023-01-01, 2023-07-01, 2024-01-01]

// Compute a day count fraction
let dcf = algebra::day_count_fraction(
    &anchor,
    &end,
    DayCount::Act365,
    Some(&cal),
    Some(AdjustRule::Following),
);
```

---

## Licenses

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
