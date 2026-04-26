//! # findates — Financial date arithmetic for Rust
//!
//! Any meaningful financial calculation requires a precise notion of time.
//! While there is extensive literature on pricing models and financial theory,
//! much less attention is given to the practical task of constructing the time
//! inputs those models depend on.
//!
//! `findates` focuses on this layer: generating correct schedules, applying
//! conventions, and computing date fractions consistently.
//!
//! ## Modules
//!
//! - [`calendar`] — [`Calendar`](calendar::Calendar) struct: weekends and holiday sets, set operations
//! - [`conventions`] — [`DayCount`](conventions::DayCount), [`AdjustRule`](conventions::AdjustRule), [`Frequency`](conventions::Frequency) enums
//! - [`algebra`] — core functions: business day checks, adjustment, day count fractions, schedule counting
//! - [`schedule`] — [`Schedule`](schedule::Schedule) and lazy [`ScheduleIterator`](schedule::ScheduleIterator)
//! - [`date`] — [`DateLike`](date::DateLike) trait implemented for [`NaiveDate`](chrono::NaiveDate)
//!
//! ## Quick start
//!
//! ```rust
//! use chrono::NaiveDate;
//! use findates::calendar::basic_calendar;
//! use findates::conventions::{AdjustRule, DayCount, Frequency};
//! use findates::schedule::Schedule;
//! use findates::algebra;
//!
//! // Build a calendar with standard Sat/Sun weekend
//! let cal = basic_calendar();
//!
//! // Adjust a Saturday to the next business day (Monday)
//! let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
//! let adj = algebra::adjust(&saturday, Some(&cal), Some(AdjustRule::Following));
//! assert_eq!(adj, NaiveDate::from_ymd_opt(2024, 3, 18).unwrap());
//!
//! // Generate a semi-annual schedule (2023 is not a leap year: exactly 365 days)
//! let anchor = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
//! let end    = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
//! let sched  = Schedule::new(Frequency::Semiannual, None, None);
//! let dates  = sched.generate(&anchor, &end).unwrap();
//! assert_eq!(dates.len(), 3); // 2023-01-01, 2023-07-01, 2024-01-01
//!
//! // Act/365 over 365 days = exactly 1.0
//! let dcf = algebra::day_count_fraction(
//!     &anchor, &end, DayCount::Act365, None, None,
//! );
//! assert!((dcf - 1.0).abs() < 1e-9);
//! ```

pub mod algebra;
pub mod calendar;
pub mod conventions;
pub mod date;
pub mod schedule;

/// Type alias for the date type used throughout the library.
pub type FinDate = chrono::NaiveDate;
