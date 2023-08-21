use findates::daycount;


fn main() {
    // Printing out values.
    let my_daycount1: daycount::DayCount;
    my_daycount1 = daycount::DayCount::Act360;
    println!("{}",my_daycount1);

    let my_daycount2: daycount::DayCount;
    my_daycount2 = daycount::DayCount::D30360;
    println!("{}",my_daycount2);

}