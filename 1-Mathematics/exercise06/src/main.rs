use std::{f32::consts::PI, io};

fn main() {
    /*
    --> Challenge 6:
        Create a program that calculates and displays the perimeter of a circle, asking the user for the radius.
    */

    println!( "Enter with the radius of the circle: " );
    let radius = loop {
        let mut buf = String::new();
        io::stdin().read_line( &mut buf ).expect( "Error read input" );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => println!( "Try Again..." )
        };
    };

    let p = 2.0 * PI * radius;
    println!( "The perimeter of the circle is {p}" );

}
