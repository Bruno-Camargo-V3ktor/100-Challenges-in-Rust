use std::io;

fn main() {
    /*
    --> Challenge 7:
        Create a program that calculates the area of a circle from its radius using the formula A = π * r².
    */

    println!( "Enter with the circle radius:" );
    let radius = loop {
        let mut buf = String::new();
        io::stdin().read_line( &mut buf ).expect( " Error read input " );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => println!( "Try Again..." )
        };
    };

    let a = std::f32::consts::PI * radius.powf(2.0);
    println!( "The area of the circle is {a}" );
}
