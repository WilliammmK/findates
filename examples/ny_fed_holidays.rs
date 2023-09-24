use std::collections::HashSet;

/// An example program used to first create a calendar with
/// the NY Federal reserve holidays and then using that calendar
/// to generate a payment schedule for a bond in USD.
/// 
/// The NY Federal Reserve Holidays as of 19th of September 2023
/// can be found on: https://www.newyorkfed.org/aboutthefed/holiday_schedule
/// and guidance on when Holidays that fall on a weekend will be observed can
/// be found on: https://www.federalpay.org/holidays
/// 

// Findates imports
use findates::algebra;
use findates::calendar::{Calendar};
use findates::conventions::*;
use findates::schedule::{self, Schedule};
// Chrono imports
use chrono::{Days,Months,Weekday,NaiveDate};



fn main() {

// ========================================================================================================================
// In the U.S. there are Fixed date Holidays and Floating date Holidays.
// Both types can be created utilizing Schedule generators and then simply
// performing a calendar union so the full set of U.S. Holidays can be contained
// in a single calendar. The Federal holiday dates are:
//     Fixed Date Holidays:

//     New Year's Day: January 1st
//     Independence Day: July 4th
//     Christmas Day: December 25th

//     Floating Holidays:

//     Thanksgiving Day: The fourth Thursday in November
//     Easter: The date varies based on the lunar calendar but typically falls on a Sunday in March or April.
//     Labor Day: The first Monday in September
//     Memorial Day: The last Monday in May
//     Veterans Day: November 11th (though observed on the nearest weekday if it falls on a weekend)
//     Columbus Day: The second Monday in October
//     Martin Luther King Jr. Day: The third Monday in January
//     Presidents' Day: The third Monday in February

// ========================================================================================================================


// Let's start by creating an empty calendar.
// We declare it mutable as we'll build it in parts.
let mut ny_fed_calendar: Calendar = Calendar::new();

// Saturdays and Sundays are non-business days there, so let's add those
// to the weekend field of our calendar:
let weekend: HashSet<Weekday> = [Weekday::Sat, Weekday::Sun].into_iter().collect();
ny_fed_calendar.add_weekends(&weekend);
println!("{:?}", ny_fed_calendar.weekend);

// Starting with the Fixed date holidays, let's create a schedule and an iterator for the NY's day:
let new_year_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
let new_year_schedule: Schedule = Schedule::new(Frequency::Annual, None, None);
let new_year_iterator = new_year_schedule.iter(new_year_day);

// We can now print out the next 5 New year dates:
println!("List of 5 NY dates: {:?}", new_year_iterator.take(4).collect::<Vec<NaiveDate>>());

// Great! But we don't need to explicitly create an iterator
// we just need the NY dates for the next 5 years, so we can just use the
// generate method.
let new_years: Vec<NaiveDate> = new_year_schedule.generate(&new_year_day, 
                                                    &new_year_day.checked_add_months(Months::new(48)).unwrap())
                                                    .expect("This should work");
println!("New Year days: {:?}", &new_years);

// Ok, but according to https://www.federalpay.org/holidays 
// Holidays that fall on a weekend should be observed on a Friday if they
// fall on a Saturday and on a Monday if they fall on a Sunday.
// To achieve that, we can make use of the Nearest Adjustment Rule:
let new_year_schedule: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let real_new_years: Vec<NaiveDate> = new_year_schedule.generate(&new_year_day, 
    &new_year_day.checked_add_months(Months::new(49)).unwrap())
    .expect("This should work");
println!("The actual observed days: {:?}", &real_new_years);

// It turns out that for the next four years NY's days will fall
// on Weekdays, so no adjustment was needed! Let's repeat this procedure
// for the rest of the fixed date holidays:
let independence_day = NaiveDate::from_ymd_opt(2023, 7,4).unwrap();





}