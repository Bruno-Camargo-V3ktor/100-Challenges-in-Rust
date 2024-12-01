

fn main() {

    /*
    --> Challenge 1:
        Write a program that asks the user for two numbers and displays their sum, subtraction, multiplication, and division
    */

    let mut buffer = String::new();

    println!( "Entre first number: " );
    std::io::stdin().read_line( &mut buffer ).expect( "Error read input" );
    buffer = buffer.trim().to_string();
    let num1: i32 = buffer.parse().expect( "Input not number" );

    buffer.clear();

    println!( "Entre second number: " );
    std::io::stdin().read_line( &mut buffer ).expect( "Error read input" );
    buffer = buffer.trim().to_string();
    let num2: i32 = buffer.parse().expect( "Input not number" );

    println!( "{num1} + {num2} = {}", num1 + num2 );
    println!( "{num1} - {num2} = {}", num1 - num2 );
    println!( "{num1} * {num2} = {}", num1 * num2 );
    println!( "{num1} / {num2} = {}", (num1 / num2) as f32 );

}
