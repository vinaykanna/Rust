fn main() {
    println!("Hello, world!");

    // bare function
    another_function();

    // function with arguments
    print_name("vinay kumar");

    // function with return value
    let result = sum(20, 14);
    println!("Result is {:?}", result);
    let result2 = three_sum(10, 20, 45);
    println!("Result 2 {:?}", result2);
}

// Defining a function
fn another_function() {
    println!("Another function");
}

// Function with parameters
fn print_name(name: &str) {
    println!("{name}");
}

// Function with return values
// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
fn three_sum(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}
