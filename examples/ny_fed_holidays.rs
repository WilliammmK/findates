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
use findates::calendar::Calendar;
use findates::conventions::*;
use findates::schedule::Schedule;
// Chrono imports
use chrono::{Weekday,NaiveDate, Datelike};




fn main() {

// ========================================================================================================================
// In the U.S. there are Fixed date Holidays and Floating date Holidays.
// Both types can be created utilizing Schedule generators and then simply
// performing a calendar union so the full set of U.S. Holidays can be contained
// in a single calendar. The Federal legal holiday dates are https://www.law.cornell.edu/uscode/text/5/6103:

//     Fixed Date Holidays:

//     New Year's Day: January 1st
//     Juneteenth National Independence Day, June 19.
//     Independence Day: July 4th
//     Veterans Day: November 11th 
//     Christmas Day: December 25th

//     Floating Holidays:

//     Thanksgiving Day: The fourth Thursday in November
//     Labor Day: The first Monday in September
//     Columbus Day: The second Monday in October
//     Martin Luther King Jr. Day: The third Monday in January
//     Washington's Birthday: The third Monday in February
//     Memorial Day: The last Monday in May

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
println!("List of NY dates: {:?}", new_year_iterator.take(4).collect::<Vec<NaiveDate>>());

// Great! But we don't need to explicitly create an iterator
// we just need the NY dates for the next 5 years, so we can just use the
// generate method.
let new_years: Vec<NaiveDate> = new_year_schedule.generate(&new_year_day, 
                                                    &algebra::checked_add_years(&new_year_day, 10).unwrap())
                                                    .expect("This should work");
println!("New Year days: {:?}", &new_years);

// Ok, but according to https://www.federalpay.org/holidays 
// Holidays that fall on a weekend should be observed on a Friday if they
// fall on a Saturday and on a Monday if they fall on a Sunday.
// To achieve that, we can make use of the Nearest Adjustment Rule:
let new_year_schedule: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let real_new_years: Vec<NaiveDate> = new_year_schedule.generate(&new_year_day, 
    &algebra::checked_add_years(&new_year_day, 10).unwrap())
    .expect("This should work");
println!("The actual observed days: {:?}", &real_new_years);

// Lets add these to our calendar:
// ny_fed_calendar.add_holidays(&real_new_years.into_iter().collect());


// It turns out that for the next four years NY's days will fall
// on Weekdays, so no adjustment was needed! Let's repeat this procedure
// for the rest of the fixed date holidays:
let independence_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 7,4).unwrap();
let independence_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let indep_days = independence_day_sch.generate(
                                                    &independence_day,
                                                    &algebra::checked_add_years(&independence_day, 10).unwrap()).unwrap();
println!("4th of july dates: {:?}", &indep_days);


// Christmas day now.
let christmas_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 12,25).unwrap();
let christmas_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let christmas_days = christmas_day_sch.generate(
                                                    &christmas_day,
                                                    &algebra::checked_add_years(&christmas_day, 10).unwrap()).unwrap();
println!("Christmas dates: {:?}", &christmas_days);

// And Veterans day.
let veterans_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 11,11).unwrap();
let veterans_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let veterans_days = veterans_day_sch.generate(
                                                    &veterans_day,
                                                    &algebra::checked_add_years(&veterans_day, 10).unwrap()).unwrap();
println!("Veteran days dates: {:?}", &veterans_days);

// And Juneteenth.
let juneteenth_day: NaiveDate = NaiveDate::from_ymd_opt(2023, 06,19).unwrap();
let juneteenth_day_sch: Schedule = Schedule::new(Frequency::Annual, Some(&ny_fed_calendar), Some(AdjustRule::Nearest));
let juneteenth_days = juneteenth_day_sch.generate(
                                                    &juneteenth_day,
                                                    &algebra::checked_add_years(&juneteenth_day, 10).unwrap()).unwrap();
println!("Juneteenth dates: {:?}", &juneteenth_days);

// Creating the floating holidays for a particular year
// is made easy using the functionalities from chrono and the use of closures.
// Let's start
let years = 2024 ..= 2033;
let thanksgiving_days: Vec<NaiveDate>;
thanksgiving_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 11, Weekday::Thu, 4).unwrap())
                                     .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                                     .collect();
println!("Thanksgiving dates: {:?}", &thanksgiving_days);

let labor_days: Vec<NaiveDate>;
labor_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 9, Weekday::Mon, 1).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Labor Day dates: {:?}", &labor_days);

let columbus_days: Vec<NaiveDate>;
columbus_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 10, Weekday::Mon, 2).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Columbus Day dates: {:?}", &columbus_days);

let mlkjr_days: Vec<NaiveDate>;
mlkjr_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 1, Weekday::Mon, 3).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("MLK Jr Day dates: {:?}", &mlkjr_days);

let washington_days: Vec<NaiveDate>;
washington_days = years.clone().map(|x| NaiveDate::from_weekday_of_month_opt(x, 2, Weekday::Mon, 3).unwrap())
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Washington's birthdate dates: {:?}", &washington_days);

// For memorial day we use a little trick with Weekdays
fn last_monday_of_may (year: i32) -> NaiveDate {
    let may_31st = NaiveDate::from_ymd_opt(year, 5, 31).unwrap();
    let delta = may_31st.weekday().num_days_from_monday() ;
    return may_31st - chrono::Duration::days(delta as i64);
}

let memorial_days: Vec<NaiveDate>;
memorial_days = years.clone().map(|x| last_monday_of_may(x))
                               .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Nearest)))
                               .collect();
println!("Memorial Day dates: {:?}", &memorial_days);

// Now let's add all of those dates to our calendar.
let mut all_holidays:Vec<NaiveDate> = itertools::concat([real_new_years
                                                    ,christmas_days
                                                    ,indep_days
                                                    ,veterans_days
                                                    ,juneteenth_days
                                                    ,thanksgiving_days
                                                    ,labor_days
                                                    ,columbus_days
                                                    ,mlkjr_days
                                                    ,washington_days
                                                    ,memorial_days]).into_iter().collect();
all_holidays.sort();
println!("Holiday Calendar: {:?}", & all_holidays);

ny_fed_calendar.add_holidays(&all_holidays.into_iter().collect());

// Now let's see how our calendar looks:
println!("NY Federal Reserve Holiday Calendar: {:?}", & ny_fed_calendar);

// Nice! Matching exactly what's described in the NY Federal Reserve site (note the
// the asterisks according to the adjustment date.)

// Now lets look at a specific financial product and how dates affect the economics of it.
// A U.S. Treasury note is a fixed rate debt instrument that pays interest semi-annually:
// https://www.treasurydirect.gov/marketable-securities/treasury-notes/
// We can look at the specific issuance auctioned on August 9th 2023:
// https://www.treasurydirect.gov/instit/annceresult/press/preanre/2023/A_20230802_2.pdf
// And the calculation details can be seem on the two links below:
// Interest calculation: https://www.ecfr.gov/current/title-31/subtitle-B/chapter-II/subchapter-A/part-356/appendix-Appendix%20B%20to%20Part%20356
// Settlement rules: https://www.ecfr.gov/current/title-31/section-356.30

// From the auction announcement we have the following relevant details:
// Term and Type of Security 10-Year Note
// Offering Amount $38,000,000,000
// Currently Outstanding $0
// CUSIP Number 91282CHT1
// Auction Date August 09, 2023
// Original Issue Date August 15, 2023
// Issue Date August 15, 2023
// Maturity Date August 15, 2033
// Dated Date August 15, 2023
// Series E-2033
// Yield Determined at Auction
// Interest Rate Determined at Auction
// Interest Payment Dates February 15 and August 15


// Ok, let's start creating the dates for our treasury bonds then:
let issue_date = NaiveDate::from_ymd_opt(2023,8,15).unwrap();
let maturity_date = NaiveDate::from_ymd_opt(2033, 8, 15).unwrap();

// The interest calculation dates will be February 15 and August 15,
// So lets create those dates until the maturity of the bond using a schedule.
let coupon_schedule = Schedule::new(Frequency::Semiannual, None, None);
let coupon_dates = coupon_schedule.generate(&issue_date, &maturity_date);
let mut coupon_dates_list = coupon_dates.unwrap().into_iter().collect::<Vec<_>>();
coupon_dates_list.sort();
println!("The coupon dates are: {:?}", &coupon_dates_list);

// Great! Those are the unadjusted coupon dates, that we can use to 
// calculate the day count fraction for the coupons. Treasury notes use a 30/360
// day count convention, so lets calculate that:
let mut dcfs: Vec<f64> = vec![  ];
for i in 0 .. (coupon_dates_list.len() - 1) {
    let dcf = algebra::day_count_fraction(coupon_dates_list.get(i).unwrap(),
                                            coupon_dates_list.get(i + 1).unwrap(), DayCount::D30360Euro, None, None);
    dcfs.push(dcf);
}

println!("Day count fractions are: {:?}", &dcfs);

// Yes! The Fractions are always 0.5 exactly like the documentation of the bond specifies!
// With these fractions we can calculate the actual interest amount to be paid, but
// what about the actual payment dates? We can adjust the coupon dates created above
// using our newly created calendar so we can check exactly when they're supposed to happen.

let settlement_dates: Vec<NaiveDate> = coupon_dates_list.into_iter()
                                                        .map(|x| algebra::adjust(&x, Some(&ny_fed_calendar), Some(AdjustRule::Following)))
                                                        .collect();

println!("The actual settlement dates are: {:?}", &settlement_dates);






}