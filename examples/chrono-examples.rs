use chrono::Datelike;
// Will use this main function for just initial exploration of chrono
// and its functionalities.
use chrono::Days;
use chrono::Months;
use chrono::NaiveDate;
use chrono::ParseError;


fn main() {
    // Creating a naive date struct
    let my_first_date: Option<NaiveDate> = NaiveDate::from_ymd_opt(2023, 08, 07);
    println!("{}", my_first_date.unwrap());
    // Date of my marriage in ordinal
    let date_from_ordinal: Option<NaiveDate> = NaiveDate::from_yo_opt(2019, 159);
    println!("{}", date_from_ordinal.unwrap());
    // Parsing from string
    let date_from_string: Result<NaiveDate, ParseError> = NaiveDate::parse_from_str("29/07/2019", "%d/%m/%Y");
    println!("{}", date_from_string.unwrap());
    // Adding days to a date
    let my_days: Days = Days::new(10);
    let moved_date: Option<NaiveDate> = my_first_date.unwrap().checked_add_days(my_days);
    //my_first_date.unwrap().checked_sub_days(my_days);
    //let my_first_date: Option<NaiveDate> = my_first_date.unwrap().checked_add_days(my_days);
    println!("moved_date Variable: ");
    println!("{}", moved_date.unwrap());
    println!("original date Variable: ");
    println!("{}", my_first_date.unwrap());

    // Successive Date
    println!("{}",date_from_string.unwrap().succ_opt().unwrap());

    // Print out weekday for a given date
    let wd: chrono::Weekday = date_from_ordinal.unwrap().weekday();
    println!("{}",wd);

    let day_of_date: u32 = date_from_string.unwrap().day();
    println!("{}",day_of_date);
    let month_of_date: u32 = date_from_string.unwrap().month();
    println!("{}",month_of_date);
    let year_of_date: i32 = date_from_string.unwrap().year();
    println!("{}",year_of_date);

    // Iterators
    println!("Days Iterator:");
    let days_it = date_from_string.unwrap().iter_days();
    for day in days_it {
        if day > my_first_date.unwrap() { break;}
        else {
            println!("Day is: {}", day);    
        }
        
    }
    // Adding a month to a 31st
    let date_31st: NaiveDate = NaiveDate::from_ymd_opt(2023, 10, 31).unwrap();
    let one_month = date_31st.checked_add_months(Months::new(1)).unwrap();
    println!("Date 31st: {}", date_31st); // It does not alter the date
    println!("Date one month: {}", one_month); // Returns a new Naive Date


}