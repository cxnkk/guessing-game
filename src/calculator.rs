use std::io;

pub fn calculator() {
    println!("Enter your first number");
    let mut input1: String = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read the input.");
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number.");

    println!("Enter your operator");
    let mut operator: String = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read input.");
    let operator = operator.trim();

    println!("Enter your second number.");
    let mut input2: String = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read the input.");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    let addition = num1 + num2;
    let multiplication = num1 * num2;
    let subtraction = num1 - num2;
    let division = num1 / num2;

    match operator {
        "+" => println!("Your result: {}", addition),
        "*" => println!("Your result: {}", multiplication),
        "-" => println!("Your result: {}", subtraction),
        "/" => println!("Your result: {}", division),
        _ => println!("Unknown operator"),
    }
}
