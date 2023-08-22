//! Day count conventions enumerations and its related functions.
//! Initial conventions here are the same as the ones in Quantlib's
//! Date and time calculations. 

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

/// Parsing error specific to DayCount
#[derive(Debug, PartialEq, Eq)]
pub struct ParseDayCountError;

/// FromStr trait implementation
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


/// Business day adjustment conventions enumerations.
/// Descriptions directly copied from quantlib docs: https://www.quantlib.org/reference/group__datetime.html 
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum  DayAdjust{
    Following,                  // Choose the first business day after the given holiday. 
    ModFollowing,               // Choose the first business day after the given holiday unless it belongs to a different month, in which case choose the first business day before the holiday. 
    Preceding,                  // Choose the first business day before the given holiday.
    ModPrecending,              // Choose the first business day before the given holiday unless it belongs to a different month, in which case choose the first business day after the holiday.
    Unadjusted,                 // Do not adjust.
    HalfMonthModFollowing,      // Choose the first business day after the given holiday unless that day crosses the mid-month (15th) or the end of month, in which case choose the first business day before the holiday. 
    Nearest                     // Choose the nearest business day to the given holiday. If both the preceding and following business days are equally far away, default to following business day. 
}


/// Display trait implementation for the DayAdjust enum.
/// Keep it consistent with the actual variant.
impl fmt::Display for DayAdjust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DayAdjust::Following => write!(f, "Following"),
            DayAdjust::ModFollowing => write!(f, "ModFollowing"),
            DayAdjust::Preceding => write!(f, "Preceding"),
            DayAdjust::ModPrecending => write!(f, "ModPreceding"),
            DayAdjust::Unadjusted => write!(f, "Unadjusted"),
            DayAdjust::HalfMonthModFollowing => write!(f, "HalfMonthModFollowing"),
            DayAdjust::Nearest => write!(f, "Nearest"),
        }
    }
}



/// Day count calculation from a start and an end date.
/// !!! Add a calendar object
pub fn day_count_fraction (start_date: NaiveDate , end_date: NaiveDate,
                           daycount: DayCount) -> f64 {


    return 3.00;
}


