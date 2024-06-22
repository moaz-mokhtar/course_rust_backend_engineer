use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("|==================================|");
    println!("| Welcome to Error Handling in Rust! |");

    match read_file_content("sample.txt") {
        Ok(content) => {
            println!("\n=====");
            println!("SUCCESS => File content: '{}'", content)
        }
        Err(e) => {
            println!("\n=====");
            println!("ERROR => Failed to read file: {}", e)
        }
    }

    // index_out_of_bounds();
    divide(3, 0);

    println!("\n\n=========");
    println!("Continue other operations");
}

// ===================
// Recoverable Errors

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    println!("\n=====");
    println!("- Reading file: '{}'", file_path);

    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// ===================
// ===================
// Unrecoverable Errors

fn index_out_of_bounds() {
    println!("\n=====");
    println!("- Inside index_out_of_bounds function");

    let v = vec![1, 2, 3];
    v[4];
}

fn divide(a: i32, b: i32) -> i32 {
    println!("\n=====");
    println!("- Inside divide function");
    println!("- Dividing a: {}, b: {}", a, b);

    if b == 0 {
        panic!("Division by zero is not allowed!");
    }
    a / b
}
