//! Error types returned by fallible findates functions.
//!
//! - [`DayCountError`] — returned by [`algebra::day_count_fraction`](crate::algebra::day_count_fraction)
//!   when called with an incompatible combination of arguments.
//! - [`BusinessDayError`] — returned by [`algebra::add_business_days`](crate::algebra::add_business_days)
//!   and [`algebra::subtract_business_days`](crate::algebra::subtract_business_days) when the
//!   start date is not a business day in the given calendar.

use std::fmt;

/// Errors returned by day count fraction calculations.
#[derive(Debug, PartialEq, Eq)]
pub enum DayCountError {
    /// Returned when [`DayCount::Bd252`](crate::conventions::DayCount::Bd252) is
    /// called without a calendar.
    MissingCalendar,
}

impl fmt::Display for DayCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayCountError::MissingCalendar => {
                write!(f, "DayCount::Bd252 requires a Calendar")
            }
        }
    }
}

impl std::error::Error for DayCountError {}

/// Errors returned by business day arithmetic functions.
#[derive(Debug, PartialEq, Eq)]
pub enum BusinessDayError {
    /// Returned when the start date is not a business day in the given calendar.
    InvalidStartDate,
}

impl fmt::Display for BusinessDayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusinessDayError::InvalidStartDate => {
                write!(f, "start date is not a business day in the given calendar")
            }
        }
    }
}

impl std::error::Error for BusinessDayError {}
