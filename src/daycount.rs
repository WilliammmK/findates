//! Day count conventions enumerations and its related functions.

use chrono::naive::NaiveDate;

/// Day count conventions enumeration. This will grow as more conventions are
/// added into scope.
pub enum DayCount {
    Act360,
    Act365,
    Bd252,
    ActAct,
    D30360,
    D30365
    
}


/// Day count calculation from a start and an end date.
/// !!! Add a calendar object
pub fn day_count_fraction (start_date: NaiveDate , end_date: NaiveDate,
                           daycount: DayCount) -> f64 {
                            

    return 3.00;
}