//! Trait for date-like types used by the library's core functions.
//!
//! [`DateLike`] is implemented for [`chrono::NaiveDate`] out of the box,
//! which is the type used throughout findates via the [`FinDate`](crate::FinDate)
//! alias.
//!
//! Implement this trait for your own date type if your codebase uses a
//! different date representation and you want to call
//! [`algebra::is_business_day`](crate::algebra::is_business_day) without
//! converting through `NaiveDate` at every call site.  All other algebra
//! functions operate on `NaiveDate` directly.

use chrono::{Datelike, Days, NaiveDate, Weekday};

/// Minimal interface over calendar dates required by
/// [`is_business_day`](crate::algebra::is_business_day).
///
/// Implemented for [`NaiveDate`] out of the box.  Implement it for your own
/// date type if you need to integrate with the library's algebra without
/// converting through `NaiveDate`.
pub(crate) trait DateLike: Copy + Ord {
    /// Gregorian year (e.g. `2024`).
    fn year(&self) -> i32;

    /// Month of the year, 1-indexed (1 = January … 12 = December).
    fn month(&self) -> u32;

    /// Day of the month, 1-indexed.
    fn day(&self) -> u32;

    /// Weekday of this date.
    fn weekday(&self) -> Weekday;

    /// Returns `self + days`, or `None` if the result would overflow the date range.
    fn add_days(&self, days: u64) -> Option<Self>
    where
        Self: Sized;

    /// Returns `self - days`, or `None` if the result would underflow the date range.
    fn sub_days(&self, days: u64) -> Option<Self>
    where
        Self: Sized;
}

impl DateLike for NaiveDate {
    fn year(&self) -> i32 {
        Datelike::year(self)
    }

    fn month(&self) -> u32 {
        Datelike::month(self)
    }

    fn day(&self) -> u32 {
        Datelike::day(self)
    }

    fn weekday(&self) -> Weekday {
        Datelike::weekday(self)
    }

    fn add_days(&self, days: u64) -> Option<Self> {
        self.checked_add_days(Days::new(days))
    }

    fn sub_days(&self, days: u64) -> Option<Self> {
        self.checked_sub_days(Days::new(days))
    }
}
