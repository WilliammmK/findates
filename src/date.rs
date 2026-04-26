//! Trait for date-like types used by the library's core functions.

use chrono::{Datelike, Days, NaiveDate, Weekday};

/// Minimal interface over calendar dates required by [`is_business_day`](crate::algebra::is_business_day).
///
/// Implemented for [`NaiveDate`] out of the box. Implement it for your own date
/// type if you need to integrate with the library's algebra without converting
/// through `NaiveDate`.
pub trait DateLike: Copy + Ord {
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
    /// ```
    /// use chrono::NaiveDate;
    /// use findates::date::DateLike;
    /// let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    /// assert_eq!(d.year(), 2024);
    /// ```
    fn year(&self) -> i32 {
        Datelike::year(self)
    }

    /// ```
    /// use chrono::NaiveDate;
    /// use findates::date::DateLike;
    /// let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    /// assert_eq!(d.month(), 3);
    /// ```
    fn month(&self) -> u32 {
        Datelike::month(self)
    }

    /// ```
    /// use chrono::NaiveDate;
    /// use findates::date::DateLike;
    /// let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    /// assert_eq!(d.day(), 15);
    /// ```
    fn day(&self) -> u32 {
        Datelike::day(self)
    }

    /// ```
    /// use chrono::{NaiveDate, Weekday};
    /// use findates::date::DateLike;
    /// let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(); // Friday
    /// assert_eq!(d.weekday(), Weekday::Fri);
    /// ```
    fn weekday(&self) -> Weekday {
        Datelike::weekday(self)
    }

    /// ```
    /// use chrono::NaiveDate;
    /// use findates::date::DateLike;
    /// let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    /// assert_eq!(d.add_days(1), NaiveDate::from_ymd_opt(2024, 3, 16));
    /// ```
    fn add_days(&self, days: u64) -> Option<Self> {
        self.checked_add_days(Days::new(days))
    }

    /// ```
    /// use chrono::NaiveDate;
    /// use findates::date::DateLike;
    /// let d = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    /// assert_eq!(d.sub_days(1), NaiveDate::from_ymd_opt(2024, 3, 14));
    /// ```
    fn sub_days(&self, days: u64) -> Option<Self> {
        self.checked_sub_days(Days::new(days))
    }
}
