fn main() {
    println!("Welcome to Functions and Closures in Rust!");

    // Function with void return value
    let num1: i32 = 50;
    let num2: i32 = 10;
    sum(num1, num2);

    // Function with a value returned
    let subtract = subtract(num1, num2);
    println!("Subtract: {}", subtract);

    // =========
    // A closure
    let sum_closure = |a: i32, b: i32| {
        let sum = a + b;
        println!("Sum (from closure): {}", sum);
    };

    println!("\n==>| Will call closure");
    sum_closure(num1, num2);
}

fn sum(a: i32, b: i32) {
    let sum = a + b;
    println!("Sum: {}", sum);
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
