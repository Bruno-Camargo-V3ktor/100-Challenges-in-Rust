use std::io;

fn main() {

    /*
    --> Challenge 3:
        Create a program that calculates and displays the arithmetic mean of three grades provided by the user.
    */

    let mut buffer = String::new();

    let num1 = loop {
        println!( "Enter first number: " );

        buffer.clear();
        io::stdin().read_line( &mut buffer ).expect( "Error read input" );

        match buffer.trim().parse::<i32>() {
            Ok( n ) => break n,
            Err( _ ) => { println!( "Input not number... Try again\n" ); continue; }
        }
    };

    let num2 = loop {
        println!( "Enter next number: " );

        buffer.clear();
        io::stdin().read_line( &mut buffer ).expect( "Error read input" );

        match buffer.trim().parse::<i32>() {
            Ok( n ) => break n,
            Err( _ ) => { println!( "Input not number... Try again\n" ); continue; }
        }
    };

    let num3 = loop {
        println!( "Enter next number: " );

        buffer.clear();
        io::stdin().read_line( &mut buffer ).expect( "Error read input" );

        match buffer.trim().parse::<i32>() {
            Ok( n ) => break n,
            Err( _ ) => { println!( "Input not number... Try again\n" ); continue; }
        }
    };

    let m = ( num1 + num2 + num3 ) as f32 / 3.0;
    println!( "The average is {}", m );

}
