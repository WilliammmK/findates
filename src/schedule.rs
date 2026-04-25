//! A date schedule that complies to a set of rules and conventions.
//!

use crate::FinDate;
use chrono::{Days, Duration, Months};

use crate::algebra::{self, adjust, checked_add_years};
use crate::calendar::Calendar;
use crate::conventions::{AdjustRule, Frequency};

/// A Schedule.
/// The Option wrapper for the calendar and adjust_rule fields allow for
/// defining a schedule without adjustments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule<'a> {
    pub frequency: Frequency,
    pub calendar: Option<&'a Calendar>,
    pub adjust_rule: Option<AdjustRule>,
}

/// Associated Schedule functions
impl<'a> Schedule<'a> {
    /// Create a new Schedule with a Frequency, Calendar and Adjust Rule
    pub fn new(
        frequency: Frequency,
        opt_calendar: Option<&'a Calendar>,
        opt_adjust_rule: Option<AdjustRule>,
    ) -> Self {
        Self {
            frequency: frequency,
            calendar: opt_calendar,
            adjust_rule: opt_adjust_rule,
        }
    }

    /// Create an iterator as a method
    pub fn iter(&self, anchor: FinDate) -> ScheduleIterator<'_> {
        ScheduleIterator {
            schedule: self,
            anchor,
        }
    }

    /// Generate a vector of dates for a given schedule with a start and an end date, including both.
    pub fn generate(
        &self,
        anchor_date: &FinDate,
        end_date: &FinDate,
    ) -> Result<Vec<FinDate>, &'static str> {
        // Check input dates
        if end_date <= anchor_date {
            return Err("Anchor date must be before end date");
        }
        // Use the iterator to collect into a Vec
        else {
            let mut res: Vec<FinDate> = vec![adjust(anchor_date, self.calendar, self.adjust_rule)];
            let iter = self.iter(*anchor_date);
            let mut res_next: Vec<FinDate> = iter
                .take_while(|x| x <= &end_date)
                .map(|x| adjust(&x, self.calendar, self.adjust_rule))
                .collect();

            res.append(&mut res_next);
            res.dedup();

            return Ok(res);
        }
    }
}

// For the case of Preceding, ModFollowing, Nearest, etc it will keep giving
// the function might simply return the same as anchor date after adjustment.
// The loop below forces that the returned date is after the anchor date.
// Should only be an issue for the Daily Frequency, but it covers all cases.
fn force_adjust(
    anchor_date: &FinDate,
    next_date: &FinDate,
    opt_calendar: Option<&Calendar>,
    opt_adjust_rule: Option<AdjustRule>,
) -> FinDate {
    let mut res: FinDate = algebra::adjust(next_date, opt_calendar, opt_adjust_rule);
    // Case where the adjustment brings the date back to the same as the anchor
    if res <= *anchor_date {
        let mut dayi = 1;
        while res <= *anchor_date {
            res = next_date.checked_add_signed(Duration::days(dayi)).unwrap_or_else(|| {
                panic!("Next Adjusted Date is out of bounds, check chrono internals for the last date available");
            });
            dayi += 1;
            res = algebra::adjust(&res, opt_calendar, opt_adjust_rule);
        }
    }
    return res;
}

// Gets the next date given an anchor date, a schedule and
// a frequency. The function will not adjust the anchor date,
// but it will adjust the next date if a calendar and adjust rule is passed.
pub fn schedule_next(anchor_date: &FinDate, frequency: Frequency) -> Option<FinDate> {
    // Calculate next for each of the Frequencies.
    match frequency {
        Frequency::Daily => {
            return anchor_date.checked_add_days(Days::new(1));
        }

        Frequency::Weekly => {
            return anchor_date.checked_add_signed(Duration::weeks(1));
        }

        Frequency::Biweekly => {
            return anchor_date.checked_add_signed(Duration::weeks(2));
        }

        Frequency::EveryFourthWeek => {
            return anchor_date.checked_add_signed(Duration::weeks(4));
        }

        Frequency::Monthly => {
            // There is no months Duration, so using Months struct from Chrono
            return anchor_date.checked_add_months(Months::new(1));
        }

        Frequency::Bimonthly => {
            return anchor_date.checked_add_months(Months::new(2));
        }

        Frequency::Quarterly => {
            return anchor_date.checked_add_months(Months::new(3));
        }

        Frequency::EveryFourthMonth => {
            return anchor_date.checked_add_months(Months::new(4));
        }

        Frequency::Semiannual => {
            return anchor_date.checked_add_months(Months::new(6));
        }

        Frequency::Annual => {
            let delta = 1;
            return checked_add_years(anchor_date, delta);
        }

        Frequency::Once => {
            return Some(*anchor_date);
        }
    }
}

/// Iterator over dates of a schedule.
/// This is an unbounded
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleIterator<'a> {
    schedule: &'a Schedule<'a>,
    anchor: FinDate,
}

impl<'a> ScheduleIterator<'a> {
    pub fn new(schedule: &'a Schedule<'a>, anchor: FinDate) -> Self {
        Self {
            schedule: schedule,
            anchor: anchor,
        }
    }
}

impl<'a> Iterator for ScheduleIterator<'a> {
    type Item = FinDate;
    fn next(&mut self) -> Option<Self::Item> {
        let res = schedule_iterator_next(self.schedule, self.anchor);
        self.anchor = res.expect("Next date for this schedule is out of bounds.");
        res
    }
}

// Next function for the Schedule iterator
fn schedule_iterator_next<'a>(schedule: &Schedule, anchor: FinDate) -> Option<FinDate> {
    schedule_next(&anchor, schedule.frequency)
}

pub fn schedule_next_adjusted<'a>(schedule: &Schedule, anchor: FinDate) -> FinDate {
    // Call next and then adjust.
    let next = schedule_next(&anchor, schedule.frequency)
        .expect("Next date for this schedule is out of bounds or malformed");
    force_adjust(&anchor, &next, schedule.calendar, schedule.adjust_rule)
}
