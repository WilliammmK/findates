//! Schedules
//! The output here can come from both methods or free functions.

use std::intrinsics::mir::Return;

use chrono::{NaiveDate, Duration};

use crate::calendar::Calendar;
use crate::conventions::{AdjustRule,DayCount, DateUnit, Frequency,Tenor};


/// A Schedule with an Anchor date
/// 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule {
    pub frequency: Frequency,
    pub calendar: Option<Calendar>,
    pub adjust_rule: Option<AdjustRule>,
}



impl Schedule {

    pub fn generate (&self, anchor_date: NaiveDate, end_date: NaiveDate ) -> Result<Vec<NaiveDate>, &'static str> {
        // Check input dates       
        if end_date <= anchor_date {
            return  Err("Anchor date must be before end date");
        } 
        // If no calendar and no adjustment just sum using the frequency.
        else if self.calendar == None && self.adjust_rule == None {
            
        }
        else {
            match self.calendar {
                
            }

            return Ok(());
        }

    }


    
}


// Auxiliary function to interpret a Frequency into a chrono Duration
fn frequency_to_duration (frequency: Frequency) -> Duration {
    match frequency {
        // !!! Use the naivedate iterators when possible
        Frequency::Daily => {
            let dur = Duration::days(1); // no timezone, so Duration::days(n) and Days::new(n) are equivalent.
            return dur;
        },
        // !!! stubs
        Frequency::Annual => {return Duration::nanoseconds(1);}
        Frequency::Bimonthly => {return Duration::nanoseconds(1);}
        Frequency::Monthly => {return Duration::nanoseconds(1);}
        Frequency::Biweekly => {return Duration::nanoseconds(1);}
        Frequency::EveryFourthMonth => {return Duration::nanoseconds(1);}
        Frequency::EveryFourthWeek => {return Duration::nanoseconds(1);}
        Frequency::Once => {return Duration::nanoseconds(1);}
        Frequency::OtherFrequency => {return Duration::nanoseconds(1);}
        Frequency::Quarterly => {return Duration::nanoseconds(1);}
        Frequency::Semiannual => {return Duration::nanoseconds(1);}
        Frequency::Weekly => {return Duration::nanoseconds(1);}


        



    }


}


/// Iterator over dates of a schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleIterator {
    value: Schedule,
}

// impl Iterator for ScheduleIterator {
    
// }