pub fn variable_and_mutability_in_rust() {
    // mutable and immutable
    #![allow(unused)]
    println!("variables and mutability");
    let variable = 5; // Here x is immutable

    println!("value of variable is {variable}");

    // variable = 6; => this would give compiler warning

    // constants
    const FOUR_HOURS_IN_SECONDS:u32=4*60*60;
    const THREE_HOURS_IN_SECONDS:i32=3*60*60;
    
    println!("FOUR_HOURS_IN_SECONDS == {FOUR_HOURS_IN_SECONDS}");

    // Shadowing
    let shadowed_variable = 100;

    let shadowed_variable =111;

    println!("Shadowed Variable is: {shadowed_variable}");
    {
        let shadowed_variable =100;
        println!("Shadowed Variable in local scope is: {shadowed_variable}");
    }

    // let spaces = "     ";
    // let spaces = spaces.len();

    // println!("length in space is {spaces}");

    let mut spaces="      ";
    let spaces = spaces.len();
}
