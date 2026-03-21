# Project: findates (Rust financial dates library)

## Purpose

This crate provides core financial date primitives:
- day count conventions
- business day calendars
- date adjustment rules
- schedule generation

It is intentionally:
- lightweight
- modular
- independent from full quant frameworks

The goal is to provide reliable building blocks that can be reused
across financial applications.

---

## Design Philosophy

- This is a **foundational library**, not a full pricing engine
- Focus on **correct primitives**, not high-level abstractions
- Prefer **composition over monolithic design**
- Avoid coupling to specific financial products

---

## Core Abstractions

The crate revolves around:

- **Date (NaiveDate from chrono)**
- **Calendar**
- **Day count conventions**
- **Business day conventions**
- **Schedule rules**

These should remain:
- orthogonal
- composable
- easy to extend

---

## Calendar Model

- A calendar defines business days
- Must support:
  - rule-based holidays (e.g. nth weekday)
  - explicit exceptions
- Should allow composition (union, intersection)

Avoid hardcoding large static holiday tables when rules can be used.

---

## Schedule Generation

- Schedules are derived from **rules**
- Prefer **lazy / iterator-based generation**
- Avoid eager allocation of large date vectors unless necessary
- Prefer lazy iterators for recurring date rules where appropriate

---

## Day Count Conventions

- Implement as explicit logic, not implicit assumptions
- Must match financial definitions precisely
- Avoid ambiguity in edge cases (e.g. leap years, irregular periods)

---

## Coding Style (Rust)

- Use idiomatic Rust
- Prefer clarity over cleverness
- Keep functions small and focused
- Avoid unnecessary traits or generics
- Do not introduce new dependencies without strong justification

---

## Performance

- Avoid unnecessary allocations
- Prefer iterators and streaming computation
- Assume usage in performance-sensitive contexts

---

## Testing

- Always include unit tests
- Focus on financial edge cases:
  - leap years
  - month boundaries
  - consecutive holidays
  - schedule boundary conditions

Correctness is more important than clever implementation.

---

## API Design

- APIs should be:
  - predictable
  - composable
  - minimal

- Do not introduce breaking changes unless explicitly requested

---

## When Generating Code

- Follow existing structure and naming
- Do not redesign abstractions unless asked
- Keep implementations minimal and correct
- Include tests when adding functionality