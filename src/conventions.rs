//! Enumerations for the standard financial market conventions.
//!
//! All three enums implement [`std::fmt::Display`] and [`std::str::FromStr`]
//! so they can be round-tripped through strings.  The string representation
//! matches the variant name exactly (case-sensitive).

use std::fmt;
use std::str::FromStr;

/// Day count conventions used when computing time fractions between two dates.
///
/// Pass one of these values to [`algebra::day_count_fraction`](crate::algebra::day_count_fraction).
///
/// # Examples
///
/// ```rust
/// use findates::conventions::DayCount;
///
/// let dc = DayCount::Act365;
/// assert_eq!(dc.to_string(), "Act365");
///
/// let parsed: DayCount = "Act360".parse().unwrap();
/// assert_eq!(parsed, DayCount::Act360);
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DayCount {
    /// Actual days divided by 360.
    ///
    /// QuantLib equivalent: `Actual360`
    Act360,

    /// Actual days divided by 365.
    ///
    /// QuantLib equivalent: `Actual365Fixed` (Standard variant)
    Act365,

    /// Actual days divided by 365, always 365 regardless of leap year.
    /// The standard convention for GBP interest rate swaps, many money
    /// market instruments, and Asian markets.
    ///
    /// Unlike [`Act365`] which uses the actual year length, this convention
    /// always divides by exactly 365 even in a leap year.
    ///
    /// QuantLib equivalent: `Actual365Fixed`
    Act365Fixed,

    /// Business days divided by 252 (Brazilian convention).
    /// Requires a [`Calendar`](crate::calendar::Calendar).
    ///
    /// QuantLib equivalent: `Business252`
    Bd252,

    /// Actual/Actual ISDA: accounts for leap years by splitting
    /// the period at year boundaries.
    ///
    /// QuantLib equivalent: `ActualActual(ActualActual::ISDA)`
    ActActISDA,

    /// 30/360 European: if either date falls on the 31st of a month
    /// it is treated as the 30th. Year of 360 days.
    ///
    /// QuantLib equivalent: `Thirty360(Thirty360::European)` /
    /// `Thirty360(Thirty360::EurobondBasis)`
    D30360Euro,

    /// 30/360 US: the standard for US corporate and municipal bonds.
    /// Also known as "30/360" or "360/360".
    ///
    /// Rules applied in order:
    /// 1. If the start date is the 31st, change it to the 30th.
    /// 2. If the start date is the last day of February, change it to the 30th.
    /// 3. If the end date is the 31st and the start date is the 30th or 31st,
    ///    change the end date to the 30th.
    /// 4. If the end date is the last day of February and the start date is
    ///    also the last day of February, change the end date to the 30th.
    ///
    /// QuantLib equivalent: `Thirty360(Thirty360::USA)`
    Thirty360US,

    /// 30/365: months of 30 days, year of 365 days.
    ///
    /// QuantLib equivalent: no direct equivalent — closest is
    /// `Thirty360` with custom year basis
    D30365,
}

impl fmt::Display for DayCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayCount::Act360 => write!(f, "Act360"),
            DayCount::Act365 => write!(f, "Act365"),
            DayCount::Act365Fixed => write!(f, "Act365Fixed"),
            DayCount::Bd252 => write!(f, "Bd252"),
            DayCount::ActActISDA => write!(f, "ActActISDA"),
            DayCount::D30360Euro => write!(f, "D30360Euro"),
            DayCount::Thirty360US => write!(f, "Thirty360US"),
            DayCount::D30365 => write!(f, "D30365"),
        }
    }
}

/// Error returned when a string cannot be parsed into a [`DayCount`].
#[derive(Debug, PartialEq, Eq)]
pub struct ParseDayCountError;

impl fmt::Display for ParseDayCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown day count convention string")
    }
}

impl FromStr for DayCount {
    type Err = ParseDayCountError;

    /// Parse a [`DayCount`] from its canonical string representation (case-sensitive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use findates::conventions::DayCount;
    ///
    /// assert_eq!("ActActISDA".parse::<DayCount>().unwrap(), DayCount::ActActISDA);
    /// assert!("actactisda".parse::<DayCount>().is_err()); // case-sensitive
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Act360" => Ok(DayCount::Act360),
            "Act365" => Ok(DayCount::Act365),
            "Act365Fixed" => Ok(DayCount::Act365Fixed),
            "Bd252" => Ok(DayCount::Bd252),
            "ActActISDA" => Ok(DayCount::ActActISDA),
            "D30360Euro" => Ok(DayCount::D30360Euro),
            "Thirty360US" => Ok(DayCount::Thirty360US),
            "D30365" => Ok(DayCount::D30365),
            _ => Err(ParseDayCountError),
        }
    }
}

/// Business day adjustment conventions.
///
/// Determines how a non-business date is moved to the nearest business day.
/// Pass one of these values to [`algebra::adjust`](crate::algebra::adjust).
///
/// Descriptions follow the [QuantLib convention reference](https://www.quantlib.org/reference/group__datetime.html).
///
/// # Examples
///
/// ```rust
/// use findates::conventions::AdjustRule;
///
/// let rule = AdjustRule::ModFollowing;
/// assert_eq!(rule.to_string(), "ModFollowing");
///
/// let parsed: AdjustRule = "Preceding".parse().unwrap();
/// assert_eq!(parsed, AdjustRule::Preceding);
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdjustRule {
    /// Choose the first business day after the given holiday.
    ///
    /// QuantLib equivalent: `BusinessDayConvention::Following`
    Following,
    /// Choose the first business day after the given holiday unless it belongs
    /// to a different month, in which case choose the first business day before.
    ///
    /// QuantLib equivalent: `BusinessDayConvention::ModifiedFollowing`
    ModFollowing,
    /// Choose the first business day before the given holiday.
    ///
    /// QuantLib equivalent: `BusinessDayConvention::Preceding`
    Preceding,
    /// Choose the first business day before the given holiday unless it belongs
    /// to a different month, in which case choose the first business day after.
    ///
    /// QuantLib equivalent: `BusinessDayConvention::ModifiedPreceding`
    ModPreceding,
    /// Do not adjust.
    ///
    /// QuantLib equivalent: `BusinessDayConvention::Unadjusted`
    Unadjusted,
    /// Like [`ModFollowing`](AdjustRule::ModFollowing) but also constrains the
    /// result to stay on the same side of the 15th of the month.
    ///
    /// QuantLib equivalent: `BusinessDayConvention::HalfMonthModifiedFollowing`
    HalfMonthModFollowing,
    /// Choose the nearest business day. When both sides are equidistant, prefer
    /// [`Following`](AdjustRule::Following).
    ///
    /// QuantLib equivalent: `BusinessDayConvention::Nearest`
    Nearest,
}

impl fmt::Display for AdjustRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdjustRule::Following => write!(f, "Following"),
            AdjustRule::ModFollowing => write!(f, "ModFollowing"),
            AdjustRule::Preceding => write!(f, "Preceding"),
            AdjustRule::ModPreceding => write!(f, "ModPreceding"),
            AdjustRule::Unadjusted => write!(f, "Unadjusted"),
            AdjustRule::HalfMonthModFollowing => write!(f, "HalfMonthModFollowing"),
            AdjustRule::Nearest => write!(f, "Nearest"),
        }
    }
}

/// Error returned when a string cannot be parsed into an [`AdjustRule`].
#[derive(Debug, PartialEq, Eq)]
pub struct ParseAdjustRuleError;

impl fmt::Display for ParseAdjustRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown adjust rule string")
    }
}

impl FromStr for AdjustRule {
    type Err = ParseAdjustRuleError;

    /// Parse an [`AdjustRule`] from its canonical string representation (case-sensitive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use findates::conventions::AdjustRule;
    ///
    /// assert_eq!("Following".parse::<AdjustRule>().unwrap(), AdjustRule::Following);
    /// assert!("following".parse::<AdjustRule>().is_err()); // case-sensitive
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Following" => Ok(AdjustRule::Following),
            "ModFollowing" => Ok(AdjustRule::ModFollowing),
            "Preceding" => Ok(AdjustRule::Preceding),
            "ModPreceding" => Ok(AdjustRule::ModPreceding),
            "Unadjusted" => Ok(AdjustRule::Unadjusted),
            "HalfMonthModFollowing" => Ok(AdjustRule::HalfMonthModFollowing),
            "Nearest" => Ok(AdjustRule::Nearest),
            _ => Err(ParseAdjustRuleError),
        }
    }
}

/// Coupon or payment frequencies.
///
/// Used by [`Schedule`](crate::schedule::Schedule) to determine how dates are
/// stepped forward in time.  Frequencies are defined relative to a one-year
/// period.
///
/// # Examples
///
/// ```rust
/// use findates::conventions::Frequency;
///
/// let f = Frequency::Semiannual;
/// assert_eq!(f.to_string(), "Semiannual");
///
/// let parsed: Frequency = "Monthly".parse().unwrap();
/// assert_eq!(parsed, Frequency::Monthly);
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Frequency {
    /// Zero coupon (single payment at maturity). For
    /// [`Schedule::generate`](crate::schedule::Schedule::generate),
    /// this returns just the end date. The iterator yields no elements
    /// after the anchor.
    ///
    /// QuantLib equivalent: `Frequency::Once` (named `Zero` in findates)
    Zero,
    /// Once a year.
    ///
    /// QuantLib equivalent: `Frequency::Annual`
    Annual,
    /// Twice a year.
    ///
    /// QuantLib equivalent: `Frequency::Semiannual`
    Semiannual,
    /// Every four months.
    EveryFourthMonth,
    /// Every three months.
    ///
    /// QuantLib equivalent: `Frequency::Quarterly`
    Quarterly,
    /// Every two months.
    ///
    /// QuantLib equivalent: `Frequency::Bimonthly`
    Bimonthly,
    /// Once a month.
    ///
    /// QuantLib equivalent: `Frequency::Monthly`
    Monthly,
    /// Every month, always landing on the last calendar day of the month.
    /// When a [`Calendar`](crate::calendar::Calendar) and
    /// [`AdjustRule`] are provided, the
    /// last calendar day is further adjusted to the nearest business day.
    EndOfMonth,
    /// Every four weeks.
    EveryFourthWeek,
    /// Every two weeks.
    ///
    /// QuantLib equivalent: `Frequency::Biweekly`
    Biweekly,
    /// Once a week.
    ///
    /// QuantLib equivalent: `Frequency::Weekly`
    Weekly,
    /// Every calendar day.
    ///
    /// QuantLib equivalent: `Frequency::Daily`
    Daily,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Frequency::Zero => write!(f, "Zero"),
            Frequency::Annual => write!(f, "Annual"),
            Frequency::Semiannual => write!(f, "Semiannual"),
            Frequency::EveryFourthMonth => write!(f, "EveryFourthMonth"),
            Frequency::Quarterly => write!(f, "Quarterly"),
            Frequency::Bimonthly => write!(f, "Bimonthly"),
            Frequency::Monthly => write!(f, "Monthly"),
            Frequency::EndOfMonth => write!(f, "EndOfMonth"),
            Frequency::EveryFourthWeek => write!(f, "EveryFourthWeek"),
            Frequency::Biweekly => write!(f, "Biweekly"),
            Frequency::Weekly => write!(f, "Weekly"),
            Frequency::Daily => write!(f, "Daily"),
        }
    }
}

/// Error returned when a string cannot be parsed into a [`Frequency`].
#[derive(Debug, PartialEq, Eq)]
pub struct ParseFrequencyError;

impl fmt::Display for ParseFrequencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown frequency string")
    }
}

impl FromStr for Frequency {
    type Err = ParseFrequencyError;

    /// Parse a [`Frequency`] from its canonical string representation (case-sensitive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use findates::conventions::Frequency;
    ///
    /// assert_eq!("Quarterly".parse::<Frequency>().unwrap(), Frequency::Quarterly);
    /// assert!("quarterly".parse::<Frequency>().is_err()); // case-sensitive
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Zero" => Ok(Frequency::Zero),
            "Annual" => Ok(Frequency::Annual),
            "Semiannual" => Ok(Frequency::Semiannual),
            "EveryFourthMonth" => Ok(Frequency::EveryFourthMonth),
            "Quarterly" => Ok(Frequency::Quarterly),
            "Bimonthly" => Ok(Frequency::Bimonthly),
            "Monthly" => Ok(Frequency::Monthly),
            "EndOfMonth" => Ok(Frequency::EndOfMonth),
            "EveryFourthWeek" => Ok(Frequency::EveryFourthWeek),
            "Biweekly" => Ok(Frequency::Biweekly),
            "Weekly" => Ok(Frequency::Weekly),
            "Daily" => Ok(Frequency::Daily),
            _ => Err(ParseFrequencyError),
        }
    }
}

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
        // Case sensitive
        let _from_str = DayCount::from_str("ActActIsda").unwrap();
    }

    #[test]
    #[should_panic]
    fn not_implemented_convention_panic_test() {
        let _from_str = DayCount::from_str("D30360ISDA").unwrap();
    }

    #[test]
    fn to_string_test() {
        let conv = AdjustRule::HalfMonthModFollowing;
        assert_eq!(conv.to_string(), "HalfMonthModFollowing");
    }

    #[test]
    fn eq_trait_test() {
        let conv = Frequency::EveryFourthMonth;
        assert_eq!(conv, Frequency::EveryFourthMonth);
    }

    #[test]
    fn bd252_roundtrip_test() {
        // Regression: "Bd2532" typo previously broke this roundtrip.
        let dc = DayCount::Bd252;
        let parsed: DayCount = dc.to_string().parse().unwrap();
        assert_eq!(dc, parsed);
    }

    #[test]
    fn all_daycount_roundtrip_test() {
        let variants = [
            DayCount::Act360,
            DayCount::Act365,
            DayCount::Act365Fixed,
            DayCount::Bd252,
            DayCount::ActActISDA,
            DayCount::D30360Euro,
            DayCount::Thirty360US,
            DayCount::D30365,
        ];
        for v in variants {
            let parsed: DayCount = v.to_string().parse().unwrap();
            assert_eq!(v, parsed);
        }
    }

    #[test]
    fn all_adjustrule_roundtrip_test() {
        let variants = [
            AdjustRule::Following,
            AdjustRule::ModFollowing,
            AdjustRule::Preceding,
            AdjustRule::ModPreceding,
            AdjustRule::Unadjusted,
            AdjustRule::HalfMonthModFollowing,
            AdjustRule::Nearest,
        ];
        for v in variants {
            let parsed: AdjustRule = v.to_string().parse().unwrap();
            assert_eq!(v, parsed);
        }
    }

    #[test]
    fn all_frequency_roundtrip_test() {
        let variants = [
            Frequency::Zero,
            Frequency::Annual,
            Frequency::Semiannual,
            Frequency::EveryFourthMonth,
            Frequency::Quarterly,
            Frequency::Bimonthly,
            Frequency::Monthly,
            Frequency::EndOfMonth,
            Frequency::EveryFourthWeek,
            Frequency::Biweekly,
            Frequency::Weekly,
            Frequency::Daily,
        ];
        for v in variants {
            let parsed: Frequency = v.to_string().parse().unwrap();
            assert_eq!(v, parsed);
        }
    }
}
