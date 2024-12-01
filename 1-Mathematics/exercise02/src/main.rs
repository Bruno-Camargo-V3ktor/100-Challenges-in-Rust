use std::io;

fn main() {

    /*
    --> Challenge 2:
        Write a program that calculates the arithmetic mean of two numbers
    */

    let mut buffer = String::new();

    println!( "Enter a first number: " );
    io::stdin().read_line( &mut buffer ).expect( "Error read input" );
    let num1 = buffer.trim().parse::<i32>().expect( "Input not number" );

    buffer.clear();

    println!( "Enter a second number: " );
    io::stdin().read_line( &mut buffer ).expect( "Error read input" );
    let num2 = buffer.trim().parse::<i32>().expect( "Input not number" );

    let m = ( num1 + num2 ) as f32 / 2.0;
    println!( "The average is {}", m );
}
