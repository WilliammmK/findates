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

// # Trait Implementations 
// Display trait for the daycount enum. Keep it consistent with the actual variant.
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

// Parsing error specific to DayCount
#[derive(Debug, PartialEq, Eq)]
pub struct ParseDayCountError;

// FromStr trait implementation
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
pub enum  AdjustRule{
    Following,                  // Choose the first business day after the given holiday. 
    ModFollowing,               // Choose the first business day after the given holiday unless it belongs to a different month, in which case choose the first business day before the holiday. 
    Preceding,                  // Choose the first business day before the given holiday.
    ModPreceding,               // Choose the first business day before the given holiday unless it belongs to a different month, in which case choose the first business day after the holiday.
    Unadjusted,                 // Do not adjust.
    HalfMonthModFollowing,      // Choose the first business day after the given holiday unless that day crosses the mid-month (15th) or the end of month, in which case choose the first business day before the holiday. 
    Nearest                     // Choose the nearest business day to the given holiday. If both the preceding and following business days are equally far away, default to following business day. 
}


// Display trait implementation for the AdjustRule enum.
// Keep it consistent with the actual variant.
impl fmt::Display for AdjustRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AdjustRule::Following             => write!(f, "Following"),
            AdjustRule::ModFollowing          => write!(f, "ModFollowing"),
            AdjustRule::Preceding             => write!(f, "Preceding"),
            AdjustRule::ModPreceding          => write!(f, "ModPreceding"),
            AdjustRule::Unadjusted            => write!(f, "Unadjusted"),
            AdjustRule::HalfMonthModFollowing => write!(f, "HalfMonthModFollowing"),
            AdjustRule::Nearest               => write!(f, "Nearest"),
        }
    }
}


// Parsing error specific to AdjustRule
#[derive(Debug, PartialEq, Eq)]
pub struct ParseAdjustRuleError;

// FromStr trait implementation for AdjustRule
impl FromStr for AdjustRule {
    type Err = ParseAdjustRuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Following"                 => Ok(AdjustRule::Following),
            "ModFollowing"              => Ok(AdjustRule::ModFollowing),
            "Preceding"                 => Ok(AdjustRule::Preceding),
            "ModPreceding"              => Ok(AdjustRule::ModPreceding),
            "Unadjusted"                => Ok(AdjustRule::Unadjusted),
            "HalfMonthModFollowing"     => Ok(AdjustRule::HalfMonthModFollowing),
            "Nearest"                   => Ok(AdjustRule::Nearest),
            _                           => Err(ParseAdjustRuleError)
        }
    }
}


/// Frequency enumeration.
/// Descriptions directly copied from quantlib docs: https://www.quantlib.org/reference/group__datetime.html 
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Frequency {
    NoFrequency, // null frequency
    Once, // only once, e.g. a zero coupon
    Annual, // once a year
    Semiannual, // twice a year
    EveryFourthMonth, // every fourth month
    Quarterly, // every fourth month
    Bimonthly, // every second month
    Monthly, // once a month
    EveryFourthWeek, // every fourth week
    Biweekly, // every second week
    Weekly, // once a week
    Daily, // once a day
    OtherFrequency // some other unknown frequency
}

// Display trait implementation for the Frequency enum.
// Keep it consistent with the actual variant.
impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Frequency::NoFrequency              => write!(f, "NoFrequency"),
            Frequency::Once                     => write!(f, "Once"),
            Frequency::Annual                   => write!(f, "Annual"),
            Frequency::Semiannual               => write!(f, "Semiannual"),
            Frequency::EveryFourthMonth         => write!(f, "EveryFourthMonth"),
            Frequency::Quarterly                => write!(f, "Quarterly"),
            Frequency::Bimonthly                => write!(f, "Bimonthly"),
            Frequency::Monthly                  => write!(f, "Monthly"),
            Frequency::EveryFourthWeek          => write!(f, "EveryFourthWeek"),
            Frequency::Biweekly                 => write!(f, "Biweekly"),
            Frequency::Weekly                   => write!(f, "Weekly"),
            Frequency::Daily                    => write!(f, "Daily"),
            Frequency::OtherFrequency           => write!(f, "OtherFrequency"),
        }
    }
}

// Parsing error specific to AdjustRule
#[derive(Debug, PartialEq, Eq)]
pub struct ParseFrequencyError;

// FromStr trait implementation for AdjustRule
impl FromStr for Frequency {
    type Err = ParseFrequencyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoFrequency"           => Ok(Frequency::NoFrequency),
            "Once"                  => Ok(Frequency::Once),
            "Annual"                => Ok(Frequency::Annual),
            "Semiannual"            => Ok(Frequency::Semiannual),
            "EveryFourthMonth"      => Ok(Frequency::EveryFourthMonth),
            "Quarterly"             => Ok(Frequency::Quarterly),
            "Bimonthly"             => Ok(Frequency::Bimonthly),
            "Monthly"               => Ok(Frequency::Monthly),
            "EveryFourthWeek"       => Ok(Frequency::EveryFourthWeek),
            "Biweekly"              => Ok(Frequency::Biweekly),
            "Weekly"                => Ok(Frequency::Weekly),
            "Daily"                 => Ok(Frequency::Daily),
            "OtherFrequency"        => Ok(Frequency::OtherFrequency),
            _                       => Err(ParseFrequencyError)
        }
    }
}


#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
/// Date units.
pub enum DateUnit {
    D,          // Calendar day
    B,          // Business day
    W,          // Week
    M,          // Month
    Y           // Year
}


#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Tenor {
    ON,
    TN,
    SN,
    _1W,
    _2W,
    _3W,
    _4W,
    _1M,
    _2M,

}



/// Day count calculation from a start and an end date.
/// !!! Add a calendar object
// pub fn day_count_fraction (start_date: NaiveDate , end_date: NaiveDate,
//                            daycount: DayCount) -> f64 {


//     return 3.00;
// }



/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        daycount: DayCount,
        adj:      AdjustRule

    }
}

