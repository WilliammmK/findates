//! Holiday Calendar objects. Essentially a list of dates that are not
//! "business days". These can be National or Local holidays usually,
//! but any other day there might be no settlement or trading.

use chrono::Weekday;
use chrono::NaiveDate;


/// A basic calendar with Saturday and Sunday as non-working days.
/// 
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Calendar {
    weekend:  Vec<Weekday>,         // Which weekdays are not good working days
    holidays: Vec<NaiveDate>        // Which days of the year are not good working days
}

// A basic calendar with Saturday and Sunday as non-working days.

