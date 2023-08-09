fn main() {
    // --- ARITHMATIC OPERATORS ---
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {}, Difference: {}", sum, difference);
    println!(" Product: {}, Quotient: {}", product, quotient);
    println!(" Truncated: {}, Remainder: {}", truncated, remainder);

    // --- LOGICAL OPERATORS ---
    let x = 10;
    let y = 12;

    // == Operator
    println!("Equal: {}", x == y);

    // != Operator
    println!("Not Equal: {}", x != y);

    // > Operator
    println!("Greater than: {}", x > y);

    // >= Operator
    println!("Greater than or equal to: {}", x >= y);

    // < Operator
    println!("Less than: {}", x < y);

    // <= Operator
    println!("Less than or equal to: {}", x <= y);

    // || Operator
    println!("Or: {}", x < y || x > y);

    // && Operator
    println!("And: {}", x < 100 && x < y);

    // ! Operator
    println!("Not: {}", !(x < 100));
}
