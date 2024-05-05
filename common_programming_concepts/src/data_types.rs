pub fn data_types()
{
    println!("About Data Types:");
    let guess:u64="420".parse().expect("not a number");
    println!("number entered is {guess}");

    
    const INTEGER_TYPES: &str =
       " INTEGER_TYPES
        Length	Signed	Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	    u128
        arch	isize	    usize";

    println!(" {INTEGER_TYPES}");

    let value:f32=2.0; //f32 
    println!("value in float is {value}");

    // let value=2.0;// f64
    // println!("value in float is {value}");
     
}

pub fn numeric_operations()
{
    let sum = 5+10;
    println!("sum is {sum}");

    let multiplication = 5*10;
    println!("multiplication is {multiplication}");

    let quotient: f64 = 56.7 / 32.2;
    println!("quotient is: {quotient}");

    let truncated = -5 / 3; // Results in -1
    println!("truncated {truncated}");

}

pub fn boolean_function()
{
        let f: bool = false; // with explicit type annotation
        println!("f is {f}")
}