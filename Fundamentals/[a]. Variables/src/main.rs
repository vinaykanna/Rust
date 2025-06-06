fn main() {
    // Variables are immutable by default
    let name: &str = "vinay kumar";
    println!("Hello {name}");

    // We can make variable mutable using `mut` keyword
    let mut a: i32 = 1;
    a = a + 1;
    println!("Number {}", a);

    // Constant variables should have the hardcoded value and type beforehand.
    const MAX_LIMIT: i32 = 300;
    println!("Max Limit is {}", MAX_LIMIT);

    // Shadowing
    // Example 1
    let shadow_var: &str = "shadow me";
    println!("{shadow_var}");
    let shadow_var: &str = "shadowed";
    println!("{shadow_var}");

    // Example 2
    let x: i32 = 5;

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
