use std::io;

fn main() {
    /*
    --> Challenge 8:
        Write a program that calculates the Δ (delta) of a quadratic equation (Δ = b² - 4 * a * c).
    */

    let mut buf = String::new();

    println!( "Enter with the value of A: " );
    let a = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read input" );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => print!( "Try again..." )
        };
    };

    println!( "Enter with the value of B: " );
    let b = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read input" );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => print!( "Try again..." )
        };
    };

    println!( "Enter with the value of C: " );
    let c = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read input" );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => print!( "Try again..." )
        };
    };

    let d = b.powf(2.0) - 4.0 * a * c;
    println!( "The value of delta is {d}" );
}
