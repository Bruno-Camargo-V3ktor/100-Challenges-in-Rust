use std::io;

fn main() {
    /*
    --> Challenge 4:
        Create a program that calculates and displays the geometric mean of three numbers provided by the user.
    */

    let mut buffer = String::new();

    println!( "Enter first number:");
    let num1 = loop {
        buffer.clear();

        io::stdin().read_line( &mut buffer ).expect( "Error read input" );
        match buffer.trim().parse::<i32>() {
            Ok( v ) => break v,
            Err(_) => { println!( "Try Again..."); }
        }
    };

    println!( "Enter second number:");
    let num2 = loop {
        buffer.clear();

        io::stdin().read_line( &mut buffer ).expect( "Error read input" );
        match buffer.trim().parse::<i32>() {
            Ok( v ) => break v,
            Err(_) => { println!( "Try Again..."); }
        }
    };

    println!( "Enter last number:");
    let num3 = loop {
        buffer.clear();

        io::stdin().read_line( &mut buffer ).expect( "Error read input" );
        match buffer.trim().parse::<i32>() {
            Ok( v ) => break v,
            Err(_) => { println!( "Try Again..."); }
        }
    };

    let m = ( ( num1 * num2 * num3 ) as f64 ).powf( 1.0/ 3.0 );
    println!( "The average is {m}" );

}
