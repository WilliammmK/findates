use chrono::{Datelike, Days, NaiveDate, Weekday};

pub trait DateLike: Copy + Ord {
    fn year(&self) -> i32;

    fn month(&self) -> u32;

    fn day(&self) -> u32;

    fn weekday(&self) -> Weekday;

    fn add_days(&self, days: u64) -> Option<Self>
    where
        Self: Sized;

    fn sub_days(&self, days: u64) -> Option<Self>
    where
        Self: Sized;
}

impl DateLike for NaiveDate {
    fn year(&self) -> i32 {
        Datelike::year(self)
    }

    fn month(&self) -> u32 {
        Datelike::month(self)
    }

    fn day(&self) -> u32 {
        Datelike::day(self)
    }

    fn weekday(&self) -> Weekday {
        Datelike::weekday(self)
    }

    fn add_days(&self, days: u64) -> Option<Self> {
        self.checked_add_days(Days::new(days))
    }

    fn sub_days(&self, days: u64) -> Option<Self> {
        self.checked_sub_days(Days::new(days))
    }
}
