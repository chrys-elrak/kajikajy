#![allow(dead_code, unused)]
mod calculator;
fn main() {
    let token = calculator::Calculator::parse("1 / 2 ");
    // println!("{:?}", token.unwrap());
    let expr = calculator::Calculator::expression(token.unwrap());
    // println!("{expr:?}");
    let value = calculator::Calculator::evaluate(expr);
    println!("{:?}", value.unwrap().unwrap());
}
