fn main() {
    println!("|==================================|");
    println!("| Welcome to Collections in Rust! |");

    // Define a vector by different ways
    // let mut names: Vec<&str> = Vec::new();
    // names.push("Moaz");
    // names.push("Ali");
    // names.push("John");

    // let names: Vec<&str> = Vec::from(["Moaz", "Ali", "John"]);

    let mut names = vec!["Moaz", "Ali", "John"];

    println!("\n=====");
    println!("Names: {:?}", names);

    let first_element = names[0];
    println!("\n=====");
    println!("\t- Frist element: {:?}", first_element);
    println!("\t- Length: {:?}", names.len());
    println!("\t- Is empty: {:?}", names.is_empty());
    println!("\t- Pop: {:?}", names.pop());

    // After popping the last element, the vector should be ["Moaz", "Ali"]
    println!("\t- Names (after pop last element): {:?}", names);
}
