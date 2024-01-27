# Variables
Variables are fundamental to almost all programming languages and play a crucial role in storing and managing data within a program.

a variable is a storage location (identified by a memory address) paired with an associated symbolic name (an identifier) that contains some known or unknown quantity of information referred to as a value. The variable name is the way to reference and manipulate this stored data in a program.

In Rust, by default, variables are immutable.

This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers.

When a variable is immutable, once a value is bound to a name, you can’t change that value.

```Rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
// This program panics and compiler throws an immutability error
```

However, you still have the option to make your variables mutable. Sometimes mutability can be very useful, and can make code more convenient to write. Although variables are immutable by default, you can make them mutable by adding **mut** in front of the variable name

```Rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## Constants
Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable

You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

Rust’s naming convention for constants is to use all uppercase with underscores between words.

```Rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```
Constants are valid for the entire time a program runs, within the scope in which they were declared. This property makes constants useful for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn, or the speed of light.

Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code. It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.

## Shadowing
In Rust, you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second. which means that the second variable is what the compiler will see when you use the name of the variable.

In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable by using the same variable’s name and repeating the use of the let

```Rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```
