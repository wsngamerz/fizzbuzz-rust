use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    println!("FizzBuzz");
    print!("Count up until >> ");
    io::stdout().flush().expect("Error flushing");

    let mut iterations = String::new();
    io::stdin()
        .read_line(&mut iterations)
        .expect("Failed to read the line");

    let iterations: u32 = iterations.trim().parse().expect("Error parsing iterations as integer");

    let mut operations = HashMap::new();
    operations.insert(3, "Fizz");
    operations.insert(5, "Buzz");

    for i in 1..=iterations {
        let mut is_multiple = false;

        for (key, value) in &operations {
            if i % key == 0 {
                print!("{}", value);
                is_multiple = true;
            }
        }

        if !is_multiple {
            print!("{i}");
        }

        print!("\n");
        io::stdout().flush().expect("Error flushing");
    }
}
