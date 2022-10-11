use std::io;

fn main() {

    println!("Bhaskara Calculator!");

        println!("a: ");
    
    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    let num1: f64 = a.trim().parse::<f64>().unwrap();
        println!("b: ");

    let mut b = String::new();

    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");

    let num2: f64 = b.trim().parse::<f64>().unwrap();

        println!("c: ");
    
    let mut c = String::new();

    io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");

    
    let num3: f64 = c.trim().parse::<f64>().unwrap();

    let delta: f64 = (num2 * num2) - (4.0 * num1 * num3);

    println!("the delta is: {}", delta);

    let square_root: f64 = delta.sqrt();

    let x1 = ((num2 * -1.0) + square_root) / (2.0 * num1);

    let x2 = ((num2 * -1.0) - square_root) / (2.0 * num1);

    println!("the x1 is: {}, and x2 is: {}", x1, x2)
            }