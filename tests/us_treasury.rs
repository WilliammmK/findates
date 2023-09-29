// Integration test recreating the U.S. Federal Holiday calendar.
// National Holidays: https://www.law.cornell.edu/uscode/text/5/6103
// NY Federal Reserve published calendar: https://www.frbservices.org/about/holiday-schedules.

use std::collections::HashSet;
use chrono::NaiveDate;

mod setup;

// Compare the generated Holidays with the list on the Federal Reserve site
#[test]
fn holiday_test () {
    // These dates have already been modified according to the asterisks in the link.
    let dates_str = [   "02/1/2023",	"01/1/2024",	"01/1/2025",	"01/1/2026",	"01/1/2027",
                                    "16/1/2023",	"15/1/2024",	"20/1/2025",	"19/1/2026",	"18/1/2027",
                                    "20/2/2023",	"19/2/2024",	"17/2/2025",	"16/2/2026",	"15/2/2027",
                                    "29/5/2023",	"27/5/2024",	"26/5/2025",	"25/5/2026",	"31/5/2027",
                                    "19/6/2023",	"19/6/2024",	"19/6/2025",	"19/6/2026",	"18/6/2027",
                                    "04/7/2023",	"04/7/2024",	"04/7/2025",	"03/7/2026",	"05/7/2027",
                                    "04/9/2023",	"02/9/2024",	"01/9/2025",	"07/9/2026",	"06/9/2027",
                                    "09/10/2023",	"14/10/2024",	"13/10/2025",	"12/10/2026",	"11/10/2027",
                                    "10/11/2023",	"11/11/2024",	"11/11/2025",	"11/11/2026",	"11/11/2027",
                                    "23/11/2023",	"28/11/2024",	"27/11/2025",	"26/11/2026",	"25/11/2027",
                                    "25/12/2023",	"25/12/2024",	"25/12/2025",	"25/12/2026",	"24/12/2027"];

    let expected_dates: HashSet<NaiveDate> = dates_str.into_iter().map(|x| NaiveDate::parse_from_str(x, "%d/%m/%Y").unwrap()).collect();

    let built_calendar = setup::calendar_setup();
    let holidays_until_27: HashSet<NaiveDate> = built_calendar.holidays.clone()
                                                .into_iter()
                                                .filter(|x| *x < NaiveDate::from_ymd_opt(2027,12,31).unwrap())
                                                .collect();

    assert_eq!(expected_dates, holidays_until_27);

}

#[test]
fn day_count_fractions_test () {
    // U.S. Calendar again
    let built_calendar = setup::calendar_setup();
    // Ouputs for the Treasury Note test
    let (_, 
        coupon_fractions,
        _) = setup::payment_schedule_setup(&built_calendar);

    // As per the the Treasury Note details, it will pay coupons semi-annualy
    // always considering a half year, i.e. the 30/360 convention. This essentially
    // means that all of the fractions should be 0.5. It is a 10 year note, so
    // there are a total of 19 coupons to be paid including the one at maturity.
    let expected = vec![0.5;20];    
    assert_eq!(expected, coupon_fractions );

}

#[test]
fn settlement_dates_test () {
    // U.S. Calendar again
    let built_calendar = setup::calendar_setup();
    // Ouputs for the Treasury Note test
    let (_, 
        _,
        settlement_dates) = setup::payment_schedule_setup(&built_calendar);

    // There is no public source availabe with all of the calculated payment
    // dates, the below was retrieved from the Security Description page
    // from Bloomber Data Services for CUSIP 91282CHT1

    let dates_str = [   "2023-08-15",   "2024-02-15",	" 2024-08-15",	" 2025-02-18",	" 2025-08-15",	" 2026-02-17"
                                ,	" 2026-08-17",	" 2027-02-16",	" 2027-08-16",	" 2028-02-15"
                                ,	" 2028-08-15",	" 2029-02-15",	" 2029-08-15",	" 2030-02-15"
                                ,	" 2030-08-15",	" 2031-02-18",	" 2031-08-15",	" 2032-02-17"
                                ,	" 2032-08-16",	" 2033-02-15",  " 2033-08-15"];

    let expected_dates: Vec<NaiveDate> = dates_str.into_iter().map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap()).collect();

    assert_eq!(expected_dates, settlement_dates);

}


