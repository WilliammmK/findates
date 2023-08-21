//! Day count conventions enumerations and its related functions.

use chrono::naive::NaiveDate;
use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

/// Day count conventions enumeration. This will grow as more conventions are
/// added into scope.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum DayCount {
    Act360,
    Act365,
    Bd252,
    ActAct,
    D30360,
    D30365    
}

/// # Trait Implementations 
/// Display trait for the daycount enum. Keep it consistent with the actual variant.
impl fmt::Display for DayCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DayCount::Act360 => write!(f, "Act360"),
            DayCount::Act365 => write!(f, "Act365"),
            DayCount::Bd252  => write!(f, "Bd252"),
            DayCount::ActAct => write!(f, "ActAct"),
            DayCount::D30360 => write!(f, "D30360"),
            DayCount::D30365 => write!(f, "D30365"),

        }
    }
}

// type Err = ParseError;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDayCountError;

// !!! Implement the from_str trait
impl FromStr for DayCount {
    type Err = ParseDayCountError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Act360" => Ok(DayCount::Act360),
            "Act365" => Ok(DayCount::Act365),
            "Bd2532" => Ok(DayCount::Bd252),
            "ActAct" => Ok(DayCount::ActAct),
            "D30360" => Ok(DayCount::D30360),
            "D30365" => Ok(DayCount::D30365),
            _        => Err(ParseDayCountError)
        }
    }
}


/// Day count calculation from a start and an end date.
/// !!! Add a calendar object
pub fn day_count_fraction (start_date: NaiveDate , end_date: NaiveDate,
                           daycount: DayCount) -> f64 {


    return 3.00;
}


