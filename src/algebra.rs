//! Core financial date functions.
//!
//! All functions are pure: they take dates and conventions by value or
//! reference and return new values without modifying their inputs or
//! maintaining any internal state.

use crate::calendar::Calendar;
use crate::conventions::{AdjustRule, DayCount};
use crate::date::DateLike;
use crate::error::DayCountError;
use chrono::{Days, NaiveDate};

/// Returns `true` if `date` is a good business day in `calendar`.
///
/// A date is a business day when it is neither a weekend day nor a holiday.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::calendar::basic_calendar;
/// use findates::algebra::is_business_day;
///
/// let cal = basic_calendar();
/// let monday = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap();
/// let saturday = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
///
/// assert!(is_business_day(&monday, &cal));
/// assert!(!is_business_day(&saturday, &cal));
/// ```
pub fn is_business_day(date: &impl DateLike, calendar: &Calendar) -> bool {
    if calendar.get_weekend().contains(&date.weekday()) {
        return false;
    }
    !calendar
        .get_holidays()
        .contains(&NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap())
}

/// Adjusts `date` to a business day according to `calendar` and `adjust_rule`.
///
/// - If `opt_calendar` is `None`, the date is returned unchanged regardless of
///   `adjust_rule`.
/// - If the date is already a business day, it is returned unchanged.
/// - If `adjust_rule` is `None` (with a calendar present), the date is also
///   returned unchanged.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::calendar::basic_calendar;
/// use findates::conventions::AdjustRule;
/// use findates::algebra::adjust;
///
/// let cal = basic_calendar();
/// // 2024-03-16 is Saturday → Following moves to Monday 2024-03-18
/// let sat = NaiveDate::from_ymd_opt(2024, 3, 16).unwrap();
/// let adj = adjust(&sat, Some(&cal), Some(AdjustRule::Following));
/// assert_eq!(adj, NaiveDate::from_ymd_opt(2024, 3, 18).unwrap());
/// ```
pub fn adjust(
    date: &NaiveDate,
    opt_calendar: Option<&Calendar>,
    adjust_rule: Option<AdjustRule>,
) -> NaiveDate {
    let calendar = match opt_calendar {
        None      => return *date,
        Some(cal) => cal,
    };

    if is_business_day(date, calendar) {
        return *date;
    }

    match adjust_rule {
        None | Some(AdjustRule::Unadjusted) => *date,

        Some(AdjustRule::Following) => add_adjust(date, calendar),

        Some(AdjustRule::ModFollowing) => {
            let adj = add_adjust(date, calendar);
            if adj.month() != date.month() { sub_adjust(date, calendar) } else { adj }
        }

        Some(AdjustRule::Preceding) => sub_adjust(date, calendar),

        Some(AdjustRule::ModPreceding) => {
            let adj = sub_adjust(date, calendar);
            if adj.month() != date.month() { add_adjust(date, calendar) } else { adj }
        }

        Some(AdjustRule::HalfMonthModFollowing) => {
            let adj = add_adjust(date, calendar);
            if adj.month() != date.month() || (date.day() <= 15 && adj.day() > 15) {
                sub_adjust(date, calendar)
            } else {
                adj
            }
        }

        Some(AdjustRule::Nearest) => {
            let fwd = add_adjust(date, calendar);
            let bwd = sub_adjust(date, calendar);
            if (fwd - *date).num_days().abs() <= (bwd - *date).num_days().abs() {
                fwd
            } else {
                bwd
            }
        }
    }
}

fn add_adjust(date: &NaiveDate, calendar: &Calendar) -> NaiveDate {
    let mut t = 1u64;
    loop {
        let candidate = date.checked_add_days(Days::new(t))
            .unwrap_or_else(|| panic!("Date out of range while searching forward for business day"));
        if is_business_day(&candidate, calendar) {
            return candidate;
        }
        t += 1;
    }
}

fn sub_adjust(date: &NaiveDate, calendar: &Calendar) -> NaiveDate {
    let mut t = 1u64;
    loop {
        let candidate = date.checked_sub_days(Days::new(t))
            .unwrap_or_else(|| panic!("Date out of range while searching backward for business day"));
        if is_business_day(&candidate, calendar) {
            return candidate;
        }
        t += 1;
    }
}

/// Generates a sorted vector of every business day from `start_date` to `end_date` inclusive.
///
/// Both endpoints are first adjusted to business days using `adjust_rule`
/// (defaults to [`Following`](AdjustRule::Following) when `None`).
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::calendar::basic_calendar;
/// use findates::algebra::bus_day_schedule;
///
/// let cal   = basic_calendar();
/// let start = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap(); // Monday
/// let end   = NaiveDate::from_ymd_opt(2024, 3, 22).unwrap(); // Friday
/// let days  = bus_day_schedule(&start, &end, &cal, None);
/// assert_eq!(days.len(), 5); // Mon – Fri
/// ```
pub fn bus_day_schedule(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    calendar: &Calendar,
    adjust_rule: Option<AdjustRule>,
) -> Vec<NaiveDate> {
    let rule = adjust_rule.or(Some(AdjustRule::Following));

    let new_start = adjust(start_date, Some(calendar), rule);
    let new_end   = adjust(end_date,   Some(calendar), rule);

    let mut schedule = vec![new_start];
    let mut prev = new_start;

    while prev < new_end {
        let mut t = 1u64;
        let mut next = adjust(
            &prev.checked_add_days(Days::new(t)).unwrap(),
            Some(calendar),
            rule,
        );
        while next <= prev {
            t += 1;
            next = adjust(
                &prev.checked_add_days(Days::new(t)).unwrap(),
                Some(calendar),
                rule,
            );
        }
        schedule.push(next);
        prev = next;
    }

    schedule
}

/// Counts the number of business days from `start_date` up to but not including `end_date`.
///
/// This follows the common financial convention of including the start date
/// and excluding the end date.  Both endpoints are adjusted as in
/// [`bus_day_schedule`].
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::calendar::basic_calendar;
/// use findates::algebra::business_days_between;
///
/// let cal   = basic_calendar();
/// let start = NaiveDate::from_ymd_opt(2024, 3, 18).unwrap(); // Monday
/// let end   = NaiveDate::from_ymd_opt(2024, 3, 22).unwrap(); // Friday
/// // Mon, Tue, Wed, Thu = 4 business days (end excluded)
/// assert_eq!(business_days_between(&start, &end, &cal, None), 4);
/// ```
pub fn business_days_between(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    calendar: &Calendar,
    adjust_rule: Option<AdjustRule>,
) -> u64 {
    let schedule = bus_day_schedule(start_date, end_date, calendar, adjust_rule);
    schedule.len() as u64 - 1
}

/// Computes the day count fraction between two dates using the given convention.
///
/// If `calendar` is `None`, no date adjustment is performed.  If `calendar` is
/// provided and `adjust_rule` is `None`, the rule defaults to
/// [`Following`](AdjustRule::Following).
///
/// If `end_date` is before `start_date` the fraction is computed on the
/// absolute time difference.
///
/// # Errors
///
/// Returns [`Err(DayCountError::MissingCalendar)`](DayCountError::MissingCalendar)
/// if `daycount` is [`Bd252`](DayCount::Bd252) and `calendar` is `None`.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::algebra::day_count_fraction;
/// use findates::conventions::DayCount;
///
/// // 2023 is not a leap year: exactly 365 days between these dates.
/// let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
/// let end   = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
///
/// // Act/365 over a full non-leap year = exactly 1.0
/// let dcf = day_count_fraction(&start, &end, DayCount::Act365, None, None).unwrap();
/// assert!((dcf - 1.0).abs() < 1e-9);
///
/// // Act/360 over 365 days
/// let dcf360 = day_count_fraction(&start, &end, DayCount::Act360, None, None).unwrap();
/// assert!((dcf360 - 365.0 / 360.0).abs() < 1e-9);
/// ```
pub fn day_count_fraction(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    daycount: DayCount,
    calendar: Option<&Calendar>,
    adjust_rule: Option<AdjustRule>,
) -> Result<f64, DayCountError> {
    let (start_adjusted, end_adjusted, some_adjust_rule, delta) = if calendar.is_none() {
        (
            *start_date,
            *end_date,
            adjust_rule,
            (*end_date - *start_date).num_days().abs(),
        )
    } else {
        let rule = if adjust_rule.is_none() {
            Some(AdjustRule::Following)
        } else {
            adjust_rule
        };
        let s = adjust(start_date, calendar, rule);
        let e = adjust(end_date, calendar, rule);
        let d = (s - e).num_days().abs();
        (s, e, rule, d)
    };

    let start_year:  i32 = start_adjusted.year();
    let start_month: i32 = start_adjusted.month() as i32;
    let mut start_day: i32 = start_adjusted.day() as i32;
    let end_year:    i32 = end_adjusted.year();
    let end_month:   i32 = end_adjusted.month() as i32;
    let mut end_day: i32 = end_adjusted.day() as i32;

    match daycount {
        DayCount::Act360 => Ok(delta as f64 / 360.0),

        DayCount::Act365 => Ok(delta as f64 / 365.0),

        DayCount::ActActISDA => {
            if start_adjusted == end_adjusted {
                return Ok(0.0);
            }
            if start_year == end_year && is_leap_year(start_year) {
                return Ok(delta as f64 / 366.0);
            }
            if start_year == end_year {
                return Ok(delta as f64 / 365.0);
            }
            if start_adjusted > end_adjusted {
                return day_count_fraction(
                    &end_adjusted,
                    &start_adjusted,
                    DayCount::ActActISDA,
                    calendar,
                    some_adjust_rule,
                );
            }
            let dcf = end_year as f64 - start_year as f64 - 1.0;
            let base1 = if is_leap_year(start_year) { 366 } else { 365 };
            let base2 = if is_leap_year(end_year)   { 366 } else { 365 };
            let dcf1 = (NaiveDate::from_ymd_opt(start_year + 1, 1, 1).unwrap()
                - start_adjusted).num_days() as f64
                / base1 as f64;
            let dcf2 = (end_adjusted
                - NaiveDate::from_ymd_opt(end_year, 1, 1).unwrap()).num_days() as f64
                / base2 as f64;
            Ok(dcf + dcf1 + dcf2)
        }

        DayCount::D30360Euro => {
            if start_day == 31 { start_day = 30; }
            if end_day   == 31 { end_day   = 30; }
            let res = 360 * (end_year - start_year)
                + 30 * (end_month - start_month)
                + (end_day - start_day);
            Ok(res as f64 / 360.0)
        }

        DayCount::D30365 => {
            let res = 360.0 * (end_year - start_year) as f64
                + 30.0 * (end_month - start_month) as f64
                + (end_day - start_day) as f64;
            Ok(res / 365.0)
        }

        DayCount::Bd252 => {
            let cal = calendar.ok_or(DayCountError::MissingCalendar)?;
            Ok(business_days_between(
                &start_adjusted,
                &end_adjusted,
                cal,
                some_adjust_rule,
            ) as f64 / 252.0)
        }
    }
}

/// Adds `years_to_add` years to `date`, returning `None` if the result is out
/// of range (e.g. Feb 29 in a non-leap target year).
///
/// chrono does not provide year arithmetic directly; this function fills the gap.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::algebra::checked_add_years;
///
/// let d = NaiveDate::from_ymd_opt(2023, 8, 15).unwrap();
/// assert_eq!(
///     checked_add_years(&d, 1),
///     NaiveDate::from_ymd_opt(2024, 8, 15)
/// );
///
/// // Feb 29 in a leap year → non-leap target year returns None
/// let leap_day = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
/// assert!(checked_add_years(&leap_day, 1).is_none());
/// ```
pub fn checked_add_years(date: &NaiveDate, years_to_add: i32) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(
        date.year() + years_to_add,
        date.month(),
        date.day(),
    )
}

fn is_leap_year(year: i32) -> bool {
    NaiveDate::from_ymd_opt(year, 2, 29).is_some()
}
