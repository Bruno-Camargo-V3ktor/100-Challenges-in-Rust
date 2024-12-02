use std::io;

fn main() {
    /*
    --> Challenge 10:
        Write a program that calculates the perimeter and the area of a triangle using the formulas P = a + b + c
        and A = (b * h) / 2, where a, b, and c are the sides of the triangle, and h is the height relative to side b.
    */

    let mut buf = String::new();

    println!( "Enter the value of the first side: " );
    let side_a = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read line" );

        match buf.trim().parse::<f32>() {
            Ok(v) => break v,
            Err(_) => println!( "Try Again..." )
        };
    };

    println!( "Enter the value of the next side: " );
    let side_b = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read line" );

        match buf.trim().parse::<f32>() {
            Ok(v) => break v,
            Err(_) => println!( "Try Again..." )
        };
    };

    println!( "Enter the value of the last side: " );
    let side_c = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read line" );

        match buf.trim().parse::<f32>() {
            Ok(v) => break v,
            Err(_) => println!( "Try Again..." )
        };
    };

    println!( "Enter the height value for side b: " );
    let height = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read line" );

        match buf.trim().parse::<f32>() {
            Ok(v) => break v,
            Err(_) => println!( "Try Again..." )
        };
    };

    let p = side_a + side_b + side_c;
    let a = (side_b * height) / 2.0;

    println!( "The perimitre of the traingulus is {p}" );
    println!( "The triangle area is {a}" );

}
