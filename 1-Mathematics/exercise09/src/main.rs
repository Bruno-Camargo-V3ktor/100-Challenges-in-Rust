use std::io;

fn main() {
    /*
    --> Challenge 9:
    Write a program that calculates the perimeter and the area of a rectangle using the formulas P = 2(l + w)
    and A = l * w, where l is the length and w is the width.
    */

    let mut buf = String::new();

    println!( "Enter the rectangle width value: " );
    let width = loop {
        buf.clear();
        io::stdin().read_line( &mut buf ).expect( "Error read line" );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => println!( "Try Again..." )
        };
    };

    println!( "Enter the value of the rectangle length: " );
    let length = loop {
        buf.clear();
        io::stdin().read_line( &mut buf ).expect( "Error read line" );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => println!( "Try Again..." )
        };
    };

    let p = 2.0 * ( width + length );
    let a = width * length;

    println!( "The perimeter of the rectangle is {p}" );
    println!( "The rectangle area is {a}" );

}
