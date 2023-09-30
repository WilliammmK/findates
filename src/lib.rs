//! # Findates: A library for dates and date operations in finance
//! 
//! Any basic calculation for financial products references a notion of time. Multiple conventions exist 
//! so that computation of time in discrete periods can be achieved. While there are multiple resources for
//! learning the theory of financial products pricing and the – often complex - models used, a lot of these 
//! fail to address the simple practicalities of getting the data to be used in those models. 
//! There is no piece of data more elemental than getting the correct schedules, day counts, and fractional 
//! periods for those calculations. Findates aims to provide the essential functionality for these common 
//! necessities when dealing with dates in a financial products context.


pub mod conventions;
pub mod calendar;
pub mod algebra;
pub mod schedule;


