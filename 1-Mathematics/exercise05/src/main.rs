use std::io;

fn main() {
    /*
    --> Challenge 5:
        Write a program that calculates an individual's BMI using the formula BMI = weight / height * height.
    */

    let mut buf = String::new();

    println!( "Enter your weight in kilograms:" );
    let weight = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read input." );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => println!( "Try Again..." )
        }
    };

    println!( "Enter your height in meters:" );
    let height = loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect( "Error read input." );

        match buf.trim().parse::<f32>() {
            Ok( v ) => break v,
            Err( _ ) => println!( "Try Again..." )
        }
    };

    let imc = weight / height.powf(2.0);
    println!( "Your BMI is {imc}" );
}
