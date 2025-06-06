fn main() {
    // If Expression
    let number = 3;

    if number < 5 {
        println!("Condition met");
    }

    // If-Else Expression (Alternative to if block)
    let number = 3;
    if number > 5 {
        println!("Condition met");
    } else {
        println!("Condition not met");
    }

    // If-Else-If Expression (handles multiple conditions)
    let number = 3;
    if number < 3 {
        println!("Condition 1 met");
    } else if number == 3 {
        println!("Condition 2 met");
    } else {
        println!("Condition 3 met");
    }

    // Using if in a let Statement
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 10 };

    println!("The value of the number is {number}");

    // Repetition with Loops
    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    let mut count: i32 = 0;

    // using break statement
    loop {
        count += 1;
        println!("looped");

        if count == 10 {
            break;
        }
    }

    // using continue statement
    loop {
        count += 1;

        if count <= 20 {
            println!("lopped again!");
            continue;
        }

        break;
    }

    // return from loop
    let result = loop {
        count += 1;

        if count >= 30 {
            break count;
        }
    };

    println!("Result {}", result);

    // Loop labels
    let mut count = 1;

    'counting_lopp: loop {
        let mut remaining = 10;

        loop {
            if count == 3 {
                break 'counting_lopp;
            }

            println!("Remaining of count {} is {}", count, remaining);

            if remaining == 0 {
                break;
            }

            remaining -= 1;
        }

        count += 1;
    }

    // Conditional loops with while
    let mut count = 0;

    while count < 10 {
        println!("Count {}", count);
        count = count + 1;
    }

    // Looping Through a Collection with for
    let numbers = [10, 20, 30, 40, 50, 60];

    for num in numbers {
        println!("The value is {num}");
    }

    // Loop through a range
    for num in (1..5).rev() {
        println!("Range element is {num}");
    }
}
