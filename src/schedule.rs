//! Frequency-based date schedule generation.
//!
//! A [`Schedule`] pairs a [`Frequency`] with an optional [`Calendar`] and
//! [`AdjustRule`].  Calling [`Schedule::iter`] returns an unbounded lazy
//! iterator; calling [`Schedule::generate`] collects dates up to a given
//! end date into a `Vec`.

use crate::FinDate;
use chrono::{Datelike, Days, Months, NaiveDate};

use crate::algebra::{self, adjust, checked_add_years};
use crate::calendar::Calendar;
use crate::conventions::{AdjustRule, Frequency};

/// A date generation rule combining a frequency, an optional calendar, and an
/// optional adjustment rule.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::schedule::Schedule;
/// use findates::conventions::Frequency;
///
/// let anchor = NaiveDate::from_ymd_opt(2023, 8, 15).unwrap();
/// let end    = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap();
/// let sched  = Schedule::new(Frequency::Semiannual, None, None);
///
/// let dates = sched.generate(&anchor, &end).unwrap();
/// assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 2, 15).unwrap());
/// assert_eq!(dates[2], end);
/// ```
///
/// ## End-of-month frequency
///
/// Use [`Frequency::EndOfMonth`] to always land on the last calendar day of
/// each month, regardless of where the anchor falls.
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::schedule::Schedule;
/// use findates::conventions::Frequency;
///
/// let anchor = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(); // Jan 31
/// let end    = NaiveDate::from_ymd_opt(2024, 4, 30).unwrap();
/// let sched  = Schedule::new(Frequency::EndOfMonth, None, None);
///
/// let dates = sched.generate(&anchor, &end).unwrap();
/// // 2024-01-31, 2024-02-29 (leap), 2024-03-31, 2024-04-30
/// assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
/// assert_eq!(dates[2], NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());
/// assert_eq!(dates[3], NaiveDate::from_ymd_opt(2024, 4, 30).unwrap());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule<'a> {
    /// The step frequency between consecutive dates.
    pub frequency: Frequency,
    /// Optional calendar used to adjust each generated date.
    pub calendar: Option<&'a Calendar>,
    /// Optional adjustment rule applied when a date falls on a non-business day.
    pub adjust_rule: Option<AdjustRule>,
}

impl<'a> Schedule<'a> {
    /// Creates a new [`Schedule`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use findates::schedule::Schedule;
    /// use findates::conventions::Frequency;
    ///
    /// let sched = Schedule::new(Frequency::Monthly, None, None);
    /// assert_eq!(sched.frequency, Frequency::Monthly);
    /// ```
    pub fn new(
        frequency: Frequency,
        opt_calendar: Option<&'a Calendar>,
        opt_adjust_rule: Option<AdjustRule>,
    ) -> Self {
        Self {
            frequency,
            calendar: opt_calendar,
            adjust_rule: opt_adjust_rule,
        }
    }

    /// Returns a lazy, unbounded iterator that yields the next date on each call.
    ///
    /// The first value yielded is the adjusted date *after* `anchor` (the anchor
    /// itself is not included).  For [`Frequency::Zero`] the iterator is
    /// immediately exhausted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use findates::schedule::Schedule;
    /// use findates::conventions::Frequency;
    ///
    /// let anchor = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
    /// let sched  = Schedule::new(Frequency::Monthly, None, None);
    ///
    /// let dates: Vec<_> = sched.iter(anchor).take(3).collect();
    /// assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()); // leap year
    /// assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 3, 29).unwrap());
    /// ```
    pub fn iter(&self, anchor: FinDate) -> ScheduleIterator<'_> {
        ScheduleIterator {
            schedule: self,
            anchor,
        }
    }

    /// Generates a `Vec` of dates from `anchor_date` to `end_date` inclusive.
    ///
    /// The anchor date is included as the first element.  Consecutive raw dates
    /// are stepped by the schedule's frequency through `end_date`, then
    /// adjusted.  Duplicate dates (which can arise when an adjustment rule moves
    /// two consecutive raw dates to the same business day) are removed.
    ///
    /// Stepping uses the **nominal (unadjusted)** date as the anchor for each
    /// subsequent interval.  This preserves date integrity for fixed-term
    /// financial instruments: an annual schedule anchored on 4 July will always
    /// step to 4 July each year before applying the adjustment rule, so a
    /// Saturday observation (Friday) never causes the next year to step from
    /// Friday and land on a different nominal date.  Use [`Schedule::iter`] when
    /// you instead want each step to begin from the previous *adjusted* date.
    ///
    /// Special case: for [`Frequency::Zero`], returns only the end date (adjusted
    /// if a calendar is set).  This represents the maturity date of a zero-coupon
    /// bond.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `end_date <= anchor_date`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::NaiveDate;
    /// use findates::calendar::basic_calendar;
    /// use findates::conventions::{AdjustRule, Frequency};
    /// use findates::schedule::Schedule;
    ///
    /// let cal    = basic_calendar();
    /// let anchor = NaiveDate::from_ymd_opt(2023, 12, 29).unwrap(); // Friday
    /// let end    = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap();
    /// let sched  = Schedule::new(Frequency::Weekly, Some(&cal), Some(AdjustRule::Following));
    ///
    /// let dates = sched.generate(&anchor, &end).unwrap();
    /// assert_eq!(dates.first().unwrap(), &anchor);
    /// ```
    pub fn generate(
        &self,
        anchor_date: &FinDate,
        end_date: &FinDate,
    ) -> Result<Vec<FinDate>, &'static str> {
        if end_date <= anchor_date {
            return Err("Anchor date must be before end date");
        }

        // Special case for Frequency::Zero: return only the adjusted end date
        if self.frequency == Frequency::Zero {
            let adjusted_end = adjust(end_date, self.calendar, self.adjust_rule);
            return Ok(vec![adjusted_end]);
        }

        let mut res = vec![adjust(anchor_date, self.calendar, self.adjust_rule)];
        let mut current = *anchor_date;
        while let Some(next) = schedule_next(&current, self.frequency) {
            if next > *end_date {
                break;
            }

            res.push(adjust(&next, self.calendar, self.adjust_rule));
            current = next;
        }
        res.dedup();
        Ok(res)
    }
}

// When Preceding/ModFollowing/Nearest adjustments could return a date ≤ anchor,
// keep stepping forward until the adjusted result is strictly after anchor.
// Returns None if the search walks off the end of the representable date range.
fn force_adjust(
    anchor_date: &FinDate,
    next_date: &FinDate,
    opt_calendar: Option<&Calendar>,
    opt_adjust_rule: Option<AdjustRule>,
) -> Option<FinDate> {
    let mut res = algebra::adjust(next_date, opt_calendar, opt_adjust_rule);
    let mut day_i = 1u64;
    while res <= *anchor_date {
        let candidate = next_date.checked_add_days(Days::new(day_i))?;
        res = algebra::adjust(&candidate, opt_calendar, opt_adjust_rule);
        day_i += 1;
    }
    Some(res)
}

// Internal building block. Returns the raw unadjusted next date for a given
// frequency. Use schedule_next_adjusted for public-facing stepping.
fn schedule_next(anchor_date: &FinDate, frequency: Frequency) -> Option<FinDate> {
    match frequency {
        Frequency::Daily => anchor_date.checked_add_days(Days::new(1)),
        Frequency::Weekly => anchor_date.checked_add_days(Days::new(7)),
        Frequency::Biweekly => anchor_date.checked_add_days(Days::new(14)),
        Frequency::EveryFourthWeek => anchor_date.checked_add_days(Days::new(28)),
        Frequency::Monthly => anchor_date.checked_add_months(Months::new(1)),
        Frequency::EndOfMonth => {
            let next = anchor_date.checked_add_months(Months::new(1))?;
            let first_of_next = if next.month() == 12 {
                NaiveDate::from_ymd_opt(next.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(next.year(), next.month() + 1, 1)
            };
            first_of_next.and_then(|d| d.pred_opt())
        }
        Frequency::Bimonthly => anchor_date.checked_add_months(Months::new(2)),
        Frequency::Quarterly => anchor_date.checked_add_months(Months::new(3)),
        Frequency::EveryFourthMonth => anchor_date.checked_add_months(Months::new(4)),
        Frequency::Semiannual => anchor_date.checked_add_months(Months::new(6)),
        Frequency::Annual => checked_add_years(anchor_date, 1),
        Frequency::Zero => None,
    }
}

/// Returns the adjusted next date after `anchor`, applying the schedule's
/// calendar and adjustment rule, or `None` if there is no next date or the
/// next date is out of range.
///
/// When successful, guarantees the result is strictly after `anchor` even when
/// an adjustment rule would otherwise move the date backwards.
///
/// Returns `None` when:
/// - The frequency has no "next" date (e.g., [`Frequency::Zero`])
/// - The next date would be out of the representable `NaiveDate` range
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::calendar::basic_calendar;
/// use findates::conventions::{AdjustRule, Frequency};
/// use findates::schedule::{Schedule, schedule_next_adjusted};
///
/// let cal    = basic_calendar();
/// let sched  = Schedule::new(Frequency::Weekly, Some(&cal), Some(AdjustRule::Following));
/// let anchor = NaiveDate::from_ymd_opt(2024, 3, 14).unwrap(); // Thursday
///
/// let next = schedule_next_adjusted(&sched, anchor).unwrap();
/// assert_eq!(next, NaiveDate::from_ymd_opt(2024, 3, 21).unwrap());
/// ```
pub fn schedule_next_adjusted(schedule: &Schedule, anchor: FinDate) -> Option<FinDate> {
    let next = schedule_next(&anchor, schedule.frequency)?;
    force_adjust(&anchor, &next, schedule.calendar, schedule.adjust_rule)
}

/// Lazy, unbounded iterator over the dates of a [`Schedule`].
///
/// Created by [`Schedule::iter`].  For [`Frequency::Zero`] the iterator is
/// immediately exhausted (returns `None` on the first call to [`next`](Iterator::next)).
///
/// Each step begins from the previous **adjusted** date, making this suitable
/// for interactive "what is the next date from today?" queries.  This differs
/// from [`Schedule::generate`], which steps from nominal (unadjusted) dates to
/// keep fixed-term schedules anchored to their intended calendar dates.
///
/// # Examples
///
/// ```rust
/// use chrono::NaiveDate;
/// use findates::schedule::Schedule;
/// use findates::conventions::Frequency;
///
/// let anchor = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// let sched  = Schedule::new(Frequency::Annual, None, None);
/// let mut it = sched.iter(anchor);
///
/// assert_eq!(it.next(), NaiveDate::from_ymd_opt(2025, 1, 1));
/// assert_eq!(it.next(), NaiveDate::from_ymd_opt(2026, 1, 1));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleIterator<'a> {
    schedule: &'a Schedule<'a>,
    anchor: FinDate,
}

impl<'a> Iterator for ScheduleIterator<'a> {
    type Item = FinDate;

    fn next(&mut self) -> Option<Self::Item> {
        let res = schedule_next_adjusted(self.schedule, self.anchor)?;
        self.anchor = res;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_of_month_schedule_next_test() {
        let sched = Schedule::new(Frequency::EndOfMonth, None, None);

        // Standard month progression (leap year Feb)
        let d = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        assert_eq!(schedule_next_adjusted(&sched, d), NaiveDate::from_ymd_opt(2024, 2, 29));

        // Non-leap February
        let d = NaiveDate::from_ymd_opt(2023, 1, 31).unwrap();
        assert_eq!(schedule_next_adjusted(&sched, d), NaiveDate::from_ymd_opt(2023, 2, 28));

        // Year boundary
        let d = NaiveDate::from_ymd_opt(2024, 11, 30).unwrap();
        assert_eq!(schedule_next_adjusted(&sched, d), NaiveDate::from_ymd_opt(2024, 12, 31));

        // Mid-month anchor still snaps to end of next month
        let d = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        assert_eq!(schedule_next_adjusted(&sched, d), NaiveDate::from_ymd_opt(2024, 2, 29));
    }
}
