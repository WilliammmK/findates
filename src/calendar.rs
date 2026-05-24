//! Holiday calendars — the set of dates that are not business days.
//!
//! A [`Calendar`] combines a set of weekend weekdays (non-working days of the week)
//! with a set of specific holiday dates.  Together these define which dates are
//! "good business days".
//!
//! Calendars can be combined with [`Calendar::union`] (useful when a trade
//! settles in two jurisdictions) or [`Calendar::intersection`] (useful when
//! only days that are holidays in *both* calendars should be excluded).

use chrono::NaiveDate;
use chrono::Weekday;
use std::borrow::Borrow;
use std::collections::HashSet;

/// A business-day calendar.
///
/// Stores two disjoint sets of non-working days:
/// - `weekend` — weekdays that are always non-working (e.g. Saturday, Sunday)
/// - `holidays` — specific calendar dates that are non-working
///
/// A date is a good business day if and only if it is neither in `weekend`
/// nor in `holidays`.
///
/// # Examples
///
/// ```rust
/// use chrono::{NaiveDate, Weekday};
/// use findates::calendar::Calendar;
///
/// let mut cal = Calendar::with_weekends([Weekday::Sat, Weekday::Sun]);
///
/// let xmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
/// cal.add_holidays([xmas]);
///
/// assert!(cal.get_holidays().contains(&xmas));
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Calendar {
    weekend: HashSet<Weekday>,
    holidays: HashSet<NaiveDate>,
}

impl Default for Calendar {
    /// Returns an empty calendar with no weekend days and no holidays.
    ///
    /// Equivalent to [`Calendar::new`].
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a calendar with Saturday and Sunday as weekend days and no holidays.
///
/// This is the most common starting point for a Western financial calendar.
///
/// # Examples
///
/// ```rust
/// use chrono::{NaiveDate, Weekday};
/// use findates::calendar::basic_calendar;
/// use findates::algebra::is_business_day;
///
/// let cal = basic_calendar();
/// let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
/// assert!(!is_business_day(&saturday, &cal));
/// let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
/// assert!(is_business_day(&monday, &cal));
/// ```
pub fn basic_calendar() -> Calendar {
    Calendar {
        weekend: [Weekday::Sat, Weekday::Sun].into_iter().collect(),
        holidays: HashSet::new(),
    }
}

/// Returns the union of a slice of calendars: a date is non-working if it is
/// non-working in *any* of the input calendars.
/// If `calendars` is empty, returns an empty calendar with no weekend
/// days and no holidays — equivalent to [`Calendar::new`].
///
/// # Examples
///
/// ```rust
/// use chrono::{NaiveDate, Weekday};
/// use findates::calendar::{Calendar, calendar_unions};
///
/// let uk = Calendar::with_weekends([Weekday::Sat, Weekday::Sun]);
///
/// let thanksgiving = NaiveDate::from_ymd_opt(2024, 11, 28).unwrap();
/// let us = Calendar::with_holidays([thanksgiving]);
///
/// let combined = calendar_unions(&[uk, us]);
/// assert!(combined.get_holidays().contains(&thanksgiving));
/// ```
pub fn calendar_unions(calendars: &[Calendar]) -> Calendar {
    let mut result = Calendar::new();
    for cal in calendars {
        result.union(cal);
    }
    result
}

impl Calendar {
    /// Construct a new empty calendar with no weekend days and no holidays.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use findates::calendar::Calendar;
    /// let cal = Calendar::new();
    /// assert!(cal.get_holidays().is_empty());
    /// assert!(cal.get_weekend().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            weekend: HashSet::new(),
            holidays: HashSet::new(),
        }
    }

    /// Construct a calendar with holiday dates and no weekend days.
    ///
    /// Accepts any iterable of holiday dates, including borrowed collections.
    /// Duplicate dates are silently ignored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use findates::calendar::Calendar;
    ///
    /// let xmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    /// let cal = Calendar::with_holidays([xmas]);
    /// assert!(cal.get_holidays().contains(&xmas));
    /// ```
    pub fn with_holidays<I>(holidays: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<NaiveDate>,
    {
        let mut calendar = Self::new();
        calendar.add_holidays(holidays);
        calendar
    }

    /// Construct a calendar with weekend weekdays and no holidays.
    ///
    /// Accepts any iterable of weekdays, including borrowed collections.
    /// Duplicate weekdays are silently ignored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Weekday;
    /// use findates::calendar::Calendar;
    ///
    /// let cal = Calendar::with_weekends([Weekday::Sat, Weekday::Sun]);
    /// assert!(cal.get_weekend().contains(&Weekday::Sat));
    /// ```
    pub fn with_weekends<I>(weekends: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Weekday>,
    {
        let mut calendar = Self::new();
        calendar.add_weekends(weekends);
        calendar
    }

    /// Returns a reference to the set of holiday dates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use findates::calendar::Calendar;
    ///
    /// let mut cal = Calendar::new();
    /// let d = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// cal.add_holidays([d]);
    /// assert!(cal.get_holidays().contains(&d));
    /// ```
    pub fn get_holidays(&self) -> &HashSet<NaiveDate> {
        &self.holidays
    }

    /// Returns a reference to the set of non-working weekdays.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Weekday;
    /// use findates::calendar::Calendar;
    ///
    /// let mut cal = Calendar::new();
    /// cal.add_weekends([Weekday::Sat, Weekday::Sun]);
    /// assert!(cal.get_weekend().contains(&Weekday::Sat));
    /// ```
    pub fn get_weekend(&self) -> &HashSet<Weekday> {
        &self.weekend
    }

    /// Adds dates to the holiday set (union with existing holidays).
    ///
    /// Accepts any iterable of holiday dates, including borrowed collections.
    /// Duplicate dates are silently ignored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use findates::calendar::Calendar;
    ///
    /// let mut cal = Calendar::new();
    /// let xmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    /// cal.add_holidays([xmas]);
    /// assert!(cal.get_holidays().contains(&xmas));
    /// ```
    pub fn add_holidays<I>(&mut self, holidays: I)
    where
        I: IntoIterator,
        I::Item: Borrow<NaiveDate>,
    {
        self.holidays
            .extend(holidays.into_iter().map(|holiday| *holiday.borrow()));
    }

    /// Adds weekdays to the weekend set (union with existing weekend days).
    ///
    /// Accepts any iterable of weekdays, including borrowed collections.
    /// Duplicate weekdays are silently ignored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Weekday;
    /// use findates::calendar::Calendar;
    ///
    /// let mut cal = Calendar::new();
    /// cal.add_weekends([Weekday::Sat, Weekday::Sun]);
    /// assert!(cal.get_weekend().contains(&Weekday::Sun));
    /// ```
    pub fn add_weekends<I>(&mut self, weekends: I)
    where
        I: IntoIterator,
        I::Item: Borrow<Weekday>,
    {
        self.weekend
            .extend(weekends.into_iter().map(|weekday| *weekday.borrow()));
    }

    /// Mutates `self` to be the union of `self` and `other`.
    ///
    /// A date is non-working in the result if it is non-working in *either*
    /// calendar.  Useful when a transaction settles in multiple jurisdictions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::{NaiveDate, Weekday};
    /// use findates::calendar::Calendar;
    ///
    /// let mut cal1 = Calendar::new();
    /// cal1.add_weekends([Weekday::Sat]);
    ///
    /// let mut cal2 = Calendar::new();
    /// cal2.add_weekends([Weekday::Sun]);
    ///
    /// cal1.union(&cal2);
    /// assert!(cal1.get_weekend().contains(&Weekday::Sat));
    /// assert!(cal1.get_weekend().contains(&Weekday::Sun));
    /// ```
    pub fn union(&mut self, other: &Calendar) {
        self.holidays = self.holidays.union(&other.holidays).cloned().collect();
        self.weekend = self.weekend.union(&other.weekend).cloned().collect();
    }

    /// Returns `true` if `date` is a good business day in this calendar.
    ///
    /// Equivalent to calling [`algebra::is_business_day`](crate::algebra::is_business_day)
    /// but more ergonomic when you already have a `Calendar` in scope.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use findates::calendar::basic_calendar;
    ///
    /// let cal = basic_calendar();
    /// let monday   = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
    /// let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
    ///
    /// assert!(cal.is_business_day(&monday));
    /// assert!(!cal.is_business_day(&saturday));
    /// ```
    pub fn is_business_day(&self, date: &chrono::NaiveDate) -> bool {
        crate::algebra::is_business_day(date, self)
    }

    /// Mutates `self` to be the intersection of `self` and `other`.
    ///
    /// A date is non-working in the result only if it is non-working in *both*
    /// calendars.  Useful when only common holidays matter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::{NaiveDate, Weekday};
    /// use findates::calendar::Calendar;
    ///
    /// let xmas  = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    /// let boxing = NaiveDate::from_ymd_opt(2024, 12, 26).unwrap();
    ///
    /// let mut cal1 = Calendar::new();
    /// cal1.add_holidays([xmas, boxing]);
    ///
    /// let mut cal2 = Calendar::new();
    /// cal2.add_holidays([xmas]);
    ///
    /// cal1.intersection(&cal2);
    /// assert!(cal1.get_holidays().contains(&xmas));
    /// assert!(!cal1.get_holidays().contains(&boxing));
    /// ```
    pub fn intersection(&mut self, other: &Calendar) {
        self.holidays = self
            .holidays
            .intersection(&other.holidays)
            .cloned()
            .collect();
        self.weekend = self.weekend.intersection(&other.weekend).cloned().collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::calendar::{self as c, Calendar};
    use chrono::{NaiveDate, Weekday};
    use std::collections::HashSet;

    #[test]
    fn add_holidays_test() {
        let mut cal = c::basic_calendar();
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let new_holidays: HashSet<NaiveDate> = [christmas_day, boxing_day].into_iter().collect();
        cal.add_holidays(&new_holidays);
        assert_eq!(cal.holidays, new_holidays);
    }

    #[test]
    fn add_weekends_test() {
        let mut cal = Calendar::new();
        let new_weekend: HashSet<Weekday> = [Weekday::Mon].into_iter().collect();
        cal.add_weekends(&new_weekend);
        assert_eq!(cal.weekend, new_weekend);
    }

    #[test]
    fn with_holidays_accepts_vec() {
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let cal = Calendar::with_holidays(vec![christmas_day, boxing_day]);

        assert!(cal.holidays.contains(&christmas_day));
        assert!(cal.holidays.contains(&boxing_day));
        assert!(cal.weekend.is_empty());
    }

    #[test]
    fn with_holidays_accepts_hashset() {
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let holidays: HashSet<NaiveDate> = [christmas_day, boxing_day].into_iter().collect();
        let cal = Calendar::with_holidays(&holidays);

        assert_eq!(cal.holidays, holidays);
        assert!(cal.weekend.is_empty());
    }

    #[test]
    fn with_holidays_accepts_array_and_iterator() {
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let array_cal = Calendar::with_holidays([christmas_day, boxing_day]);
        let iterator_cal = Calendar::with_holidays([christmas_day, boxing_day].into_iter().take(1));

        assert_eq!(array_cal.holidays.len(), 2);
        assert!(array_cal.holidays.contains(&christmas_day));
        assert!(array_cal.holidays.contains(&boxing_day));
        assert_eq!(iterator_cal.holidays, [christmas_day].into_iter().collect());
    }

    #[test]
    fn with_weekends_accepts_array() {
        let cal = Calendar::with_weekends([Weekday::Sat, Weekday::Sun]);

        assert_eq!(
            cal.weekend,
            [Weekday::Sat, Weekday::Sun].into_iter().collect()
        );
        assert!(cal.holidays.is_empty());
    }

    #[test]
    fn with_weekends_accepts_vec_and_hashset() {
        let vec_cal = Calendar::with_weekends(vec![Weekday::Sat, Weekday::Sun]);
        let weekend: HashSet<Weekday> = [Weekday::Sat, Weekday::Sun].into_iter().collect();
        let hashset_cal = Calendar::with_weekends(&weekend);

        assert_eq!(vec_cal.weekend, weekend);
        assert_eq!(hashset_cal.weekend, weekend);
    }

    #[test]
    fn with_weekends_accepts_iterator() {
        let cal = Calendar::with_weekends(
            [Weekday::Sat, Weekday::Sun]
                .into_iter()
                .filter(|weekday| *weekday == Weekday::Sat),
        );

        assert_eq!(cal.weekend, [Weekday::Sat].into_iter().collect());
        assert!(cal.holidays.is_empty());
    }

    #[test]
    fn calendar_constructors_accept_empty_iterators() {
        let holidays = Calendar::with_holidays(std::iter::empty::<NaiveDate>());
        let weekends = Calendar::with_weekends(std::iter::empty::<Weekday>());

        assert!(holidays.holidays.is_empty());
        assert!(holidays.weekend.is_empty());
        assert!(weekends.holidays.is_empty());
        assert!(weekends.weekend.is_empty());
    }

    #[test]
    fn calendar_iterators_ignore_duplicates() {
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let mut cal = Calendar::with_holidays([christmas_day, christmas_day]);
        let weekends = Calendar::with_weekends([Weekday::Sun, Weekday::Sun]);

        cal.add_holidays([boxing_day, boxing_day]);
        cal.add_weekends([Weekday::Sat, Weekday::Sat]);
        assert_eq!(cal.holidays.len(), 2);
        assert_eq!(cal.weekend.len(), 1);
        assert_eq!(weekends.weekend.len(), 1);
    }

    #[test]
    fn add_methods_accept_empty_iterators() {
        let mut cal = Calendar::new();

        cal.add_holidays(std::iter::empty::<NaiveDate>());
        cal.add_weekends(std::iter::empty::<Weekday>());

        assert!(cal.holidays.is_empty());
        assert!(cal.weekend.is_empty());
    }

    #[test]
    fn get_holidays_test() {
        let mut cal = c::basic_calendar();
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();
        let new_holidays: HashSet<NaiveDate> = [christmas_day, boxing_day].into_iter().collect();
        cal.add_holidays(&new_holidays);
        assert_eq!(cal.get_holidays(), &new_holidays);
    }

    #[test]
    fn get_weekend_test() {
        let mut cal = Calendar::new();
        let new_weekend: HashSet<Weekday> = [Weekday::Mon].into_iter().collect();
        cal.add_weekends(&new_weekend);
        assert_eq!(cal.get_weekend(), &new_weekend);
    }

    #[test]
    fn calendar_union_test() {
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();

        let mut cal1 = Calendar::new();
        cal1.add_weekends([Weekday::Sat]);
        cal1.add_holidays([christmas_day]);

        let mut cal2 = Calendar::new();
        cal2.add_weekends([Weekday::Sun]);
        cal2.add_holidays([boxing_day]);

        let mut expected = c::basic_calendar();
        expected.add_holidays([christmas_day, boxing_day]);

        cal1.union(&cal2);
        assert_eq!(cal1, expected);
    }

    #[test]
    fn calendar_intersection_test() {
        let christmas_day = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let boxing_day = NaiveDate::from_ymd_opt(2023, 12, 26).unwrap();

        let mut cal1 = Calendar::new();
        cal1.add_weekends([Weekday::Sun]);
        cal1.add_holidays([christmas_day]);

        let mut cal2 = Calendar::new();
        cal2.add_weekends([Weekday::Sun]);
        cal2.add_holidays([christmas_day, boxing_day]);

        let mut expected = Calendar::new();
        expected.add_weekends([Weekday::Sun]);
        expected.add_holidays([christmas_day]);

        cal1.intersection(&cal2);
        assert_eq!(cal1, expected);
    }

    #[test]
    fn default_is_empty_test() {
        let cal = Calendar::default();
        assert!(cal.get_holidays().is_empty());
        assert!(cal.get_weekend().is_empty());
    }

    #[test]
    fn calendar_unions_test() {
        let xmas = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        let mut cal1 = Calendar::new();
        cal1.add_weekends([Weekday::Sat]);
        let mut cal2 = Calendar::new();
        cal2.add_holidays([xmas]);

        let combined = c::calendar_unions(&[cal1, cal2]);
        assert!(combined.get_weekend().contains(&Weekday::Sat));
        assert!(combined.get_holidays().contains(&xmas));
    }
}
