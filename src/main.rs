#![allow(dead_code, unused)]
mod calculator;
use crate::calculator::Calculator;

fn main() -> Result<(),  Box<dyn std::error::Error>> {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let token = Calculator::parse(input);
                if token.is_err() {
                    println!("{:?}", token.err().unwrap());
                    continue;
                }
                let expr = Calculator::expression(token.unwrap());
                if let Some(v) = Calculator::evaluate(expr) {
                    println!("{:?}", v.unwrap());
                }
            }
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        }
    }
}
