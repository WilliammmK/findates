use findates::conventions;


fn main() {
    // Printing out values.
    let my_daycount1: conventions::DayCount;
    my_daycount1 = conventions::DayCount::Act360;
    println!("{}",my_daycount1);

    let my_daycount2: conventions::DayCount;
    my_daycount2 = conventions::DayCount::D30360;
    println!("{}",my_daycount2);

}