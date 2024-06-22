fn main() {
    println!("|==================================|");
    println!("| Welcome to Control Flow in Rust! |");

    // _if_else();
    // _age_if_else();
    // _loop_in_rust();
    // _while_in_rust();
    // _for_in_rust();
    // _match_in_rust_age();
    // _match_in_rust_days();
    // _match_in_rust_option();
}
fn _match_in_rust_option() {
    let age: Option<i32> = Some(100);

    match age {
        Some(age) => {
            println!("Got an age: {}", age);
            match age {
                0..=18 => println!("You are young"),
                19..=40 => println!("You are an adult"),
                41..=150 => println!("You are old"),
                151..=200 => println!("It seems you are in a historical age"),
                _ => println!("Invalid age"),
            }
        }
        None => println!("No age"),
    }
}

fn _match_in_rust_age() {
    let age = -5000000;

    match age {
        0..=18 => println!("You are young"),
        19..=40 => println!("You are an adult"),
        41..=150 => println!("You are old"),
        151..=200 => println!("It seems you are in a historical age"),
        _ => println!("Invalid age"),
    }
}

fn _match_in_rust_days() {
    enum Days {
        Friday,
        Saturday,
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
    }

    let today = Days::Friday;

    match today {
        Days::Friday => println!("Today is Friday"),
        Days::Saturday => println!("Today is Saturday"),
        Days::Sunday => println!("Today is Sunday"),
        Days::Monday => println!("Today is Monday"),
        Days::Tuesday => println!("Today is Tuesday"),
        Days::Wednesday => println!("Today is Wednesday"),
        Days::Thursday => println!("Today is Thursday"),
    }
}

// ====

fn _if_else() {
    let number = 12;

    if number > 10 {
        println!("Number is greater than 10");
    } else {
        println!("Number is less than or equal to 10");
    }
}

fn _age_if_else() {
    let age = -1;

    if age > 0 && age < 18 {
        println!("You are young");
    } else if age >= 18 && age < 40 {
        println!("You are an adult");
    } else if age > 40 {
        println!("You are old");
    } else {
        println!("Invalid age");
    }
}

fn _loop_in_rust() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 10 {
            println!("Breaking out of the loop after 10 iterations");
            break;
        }

        // sleep for a second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn _while_in_rust() {
    let mut counter = 0;

    while counter < 10 {
        counter += 1;
        println!("Counter: {}", counter);
    }

    println!("==|> While loop finished");
}

fn _for_in_rust() {
    for number in 1..=10 {
        println!("\n# Normal Number: {}", number);
    }

    // Loop in reverse
    for number in (1..=10).rev() {
        println!("\n# Reversed Number: {}", number);
    }

    // Loop over an array
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers {
        println!("\n# Array Number: {}", number);
    }
}
