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
