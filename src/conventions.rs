//! Enumerations for the different market conventions and its related functions.
 

use std::fmt;
use std::str::FromStr;

/// Day count conventions enumeration. This will grow as more conventions are
/// added into scope.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum DayCount {
    Act360,
    Act365,
    Bd252,
    ActActISDA,
    D30360Euro, 
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
            DayCount::ActActISDA => write!(f, "ActActISDA"),
            DayCount::D30360Euro => write!(f, "D30360Euro"),
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
            "Act360"     => Ok(DayCount::Act360),
            "Act365"     => Ok(DayCount::Act365),
            "Bd2532"     => Ok(DayCount::Bd252),
            "ActActISDA" => Ok(DayCount::ActActISDA),
            "D30360Euro" => Ok(DayCount::D30360Euro),
            "D30365"     => Ok(DayCount::D30365),
            _            => Err(ParseDayCountError)
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
/// These are all in reference to a 1 year period, i.e
/// Descriptions directly copied from quantlib docs: https://www.quantlib.org/reference/group__datetime.html 
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Frequency {
    /// only once, e.g. a zero coupon
    Once, 
    /// once a year
    Annual, 
    /// twice a year
    Semiannual, 
    /// every fourth month
    EveryFourthMonth, 
    /// every fourth month
    Quarterly, 
    /// every second month
    Bimonthly,
    /// once a month 
    Monthly, 
    /// every fourth week
    EveryFourthWeek, 
    /// every second week
    Biweekly,
    /// once a week 
    Weekly, 
    /// once a day
    Daily,
}

// Display trait implementation for the Frequency enum.
// Keep it consistent with the actual variant.
impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
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
            _                       => Err(ParseFrequencyError)
        }
    }
}




/// Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn from_string_parse_test() {
        let from_str = DayCount::from_str("ActActISDA");
        assert_eq!(DayCount::ActActISDA, from_str.unwrap());
    }

    #[test]
    #[should_panic]
    fn incorrect_string_panic_test() {
        // Case sensitive, so panics if case does not match
        let _from_str = DayCount::from_str("ActActIsda").unwrap();

        let _from_str = DayCount::from_str("D30360ISDA").unwrap();
    }

    #[test]
    #[should_panic]
    fn not_implemented_convention_panic_test() {
        // Panics as the convention has not been implemented yet.
        let _from_str = DayCount::from_str("D30360ISDA").unwrap();
    }

    #[test]
    fn to_string_test () {
        let conv = AdjustRule::HalfMonthModFollowing;
        assert_eq!(conv.to_string(), "HalfMonthModFollowing");
    }

    #[test]
    fn eq_trait_test () {
        let conv = Frequency::EveryFourthMonth;
        assert_eq!(conv, Frequency::EveryFourthMonth);
    }




}

