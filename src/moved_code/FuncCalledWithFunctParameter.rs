// Define a function to process two numbers
fn process_numbers(a: i32, b: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(a, b)
}

// Define some functions to use as operations
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let a = 3;
    let b = 4;

    // Call `process_numbers` with the `add` function
    let sum = process_numbers(a, b, add);
    println!("{} + {} = {}", a, b, sum);

    // Call `process_numbers` with the `multiply` function
    let product = process_numbers(a, b, multiply);
    println!("{} * {} = {}", a, b, product);
}
