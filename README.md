# Findates

**Findates** is a Rust library for handling dates in financial applications — including schedules, business day adjustments, and day count conventions.

Any meaningful financial calculation relies on a notion of time. While there is extensive literature on pricing models and financial theory, much less attention is given to the practical task of constructing the time inputs those models depend on.

Findates focuses on this layer:
> generating correct schedules, applying conventions, and computing time/dates consistently.

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
- an implementation aligned with **Rust’s strengths** (type safety, iterators, composability)

---

## Design Approach

The library follows a deliberately simple structure, separating the main concepts involved in financial date handling:

- `calendar`: definition of working days (weekends and holidays)
- `conventions`: financial rules (adjustment rules, day count conventions)
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
- Date adjustment using standard financial conventions  
- Frequency-based schedule generation  
- Day count fraction calculations (e.g. Act/360, Act/365, 30/360, BD252)  
- Iterator-based schedules for flexible usage  

---

## Example

```rust
use chrono::NaiveDate;
use findates::calendar::basic_calendar;
use findates::conventions::{AdjustRule, DayCount};
use findates::algebra;

let cal = basic_calendar();

let start = NaiveDate::from_ymd_opt(2023, 9, 30).unwrap();
let end   = NaiveDate::from_ymd_opt(2023, 12, 24).unwrap();

// Adjust dates according to business day rules
let adj_start = algebra::adjust(&start, Some(&cal), Some(AdjustRule::Following));
let adj_end   = algebra::adjust(&end,   Some(&cal), Some(AdjustRule::Following));

// Compute day count fraction
let dcf = algebra::day_count_fraction(
    &adj_start,
    &adj_end,
    DayCount::Act360,
    Some(&cal),
    Some(AdjustRule::Following),
);
```
---

### Licenses

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


