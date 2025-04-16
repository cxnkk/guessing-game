use core::str;
use std::io;

enum Operator {
    Add,
    Mul,
    Sub,
    Div,
}

impl std::str::FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mul),
            "-" => Ok(Operator::Sub),
            "/" => Ok(Operator::Div),
            &_ => Err("Fail.".to_string()),
        }
    }
}

pub fn calculator() {
    println!("Enter your first number");
    let mut input1: String = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let num1: f64 = input1.trim().parse().unwrap();

    println!("Enter your operator");
    let mut operator: String = String::new();
    io::stdin().read_line(&mut operator).unwrap();
    let operator: Operator = operator.trim().parse().unwrap();

    println!("Enter your second number.");
    let mut input2: String = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let num2: f64 = input2.trim().parse().unwrap();

    match operator {
        Operator::Add => {
            let addition = num1 + num2;
            println!("Your result: {}", addition)
        }
        Operator::Mul => {
            let multiplication = num1 * num2;
            println!("Your result: {}", multiplication)
        }
        Operator::Sub => {
            let subtraction = num1 - num2;
            println!("Your result: {}", subtraction)
        }
        Operator::Div => {
            let division = num1 / num2;
            println!("Your result: {}", division)
        }
    }
}
