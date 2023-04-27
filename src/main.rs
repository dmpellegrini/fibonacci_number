use std::io;

fn main() {
    println!("Welcome to the fibonacci sequence");

    println!("This program will generate the nth fibonacci number");

    println!("Simply enter a number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number
        .trim()
        .parse()
        .expect("Please type a number!");

    let fib_number: i32 = fibonacci(number);

    println!("The {number} number in the sequence is {fib_number}");
}

fn fibonacci(number: i32) -> i32 {
    if number <= 2{
        1
    }else{
        fibonacci(number - 1) + fibonacci(number - 2)
    }
}
