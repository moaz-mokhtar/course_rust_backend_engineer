const PI: f32 = 3.14;

fn main() {
    println!("Welcome to illustrate Data types in Rust!");

    let name: &str = "Khalid"; // Default and Immutable
                               // name = "Khalid (edited)"
                               // println!("Name: {}", name);

    // ===

    let mut age: i32 = 15; // Mutable variable
                           // println!("Age (initial): {}", age);

    age = 20;
    // println!("Age (edited): {}", age);

    // ===

    println!("PI: {}", PI);

    // PI = 4.5;
    // println!("PI: {}", PI);

    const IPI: f32 = 3.14;
    println!("PI (internal scope): {}", IPI);
}
