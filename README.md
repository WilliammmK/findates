# findates
A Rust crate for dealing with dates in finance.

findates provides core building blocks for working with dates in financial contexts, including calendars, business day conventions, day count calculations, and schedule generation.

It is designed to be simple, composable, and independent from larger quantitative finance frameworks.

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

findates focuses on solving these foundational problems in a clean, reusable way.

---

## Features

- Business day calendars  
- Holiday rules and exceptions  
- Business day adjustment conventions  
- Day count conventions  
- Schedule generation from rules  

---

## Non-goals

findates is **not**:

- a full quantitative finance library  
- a pricing or risk engine  
- a replacement for frameworks like QuantLib  

It is intended to be used as a **foundation layer** within larger systems.

---

## Example Use Cases

- Financial product cash flow generation  
- Settlement and payment date calculations  
- Treasury and back-office systems  
- Quantitative models requiring date schedules  
- Fintech infrastructure services  

---

## Project Status

This project is under active development.

The API may evolve as the design is refined, but stability and clarity are priorities.

---

## Documentation

Detailed usage examples and API documentation are available via Rust docs:

```bash
cargo doc --open
```

---

## Licenses

This project is dual-licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option. Choose whichever license works best for your use case.
