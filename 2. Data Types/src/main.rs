fn main() {
    // --- SCALAR TYPES ---
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

    // --- INTEGER TYPES ---
    // Signed - positive or negative
    let _x: i8 = 100;
    let _x: i16 = 200;
    let _x: i32 = 400;
    let x: i64 = -1000;
    println!("{x}");

    // Unsigned - only positive
    let _y: u8 = 100;
    let _y: u16 = 200;
    let _y: u32 = 400;
    let y: u64 = 1000;
    println!("{y}");

    // --- FLOAT TYPES ---
    // All float types are signed
    // The f32 type is a single-precision float, and f64 has double precision.
    let _z: f64 = 1.65;
    let z: f32 = 1.235;
    println!("{z}");

    // --- BOOLEAN TYPE ---
    let t: bool = true;
    let f: bool = false;
    println!("{t},{f}");

    // --- CHARACTER TYPE ---
    let w: char = 'W';
    let heart_eyed_cat: char = '😻';
    println!("{w}, {heart_eyed_cat}");

    // --- COMPOUND TYPES ---
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // --- TUPLE TYPE ---
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value
    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (first, second, third) = tup;
    println!(
        "Tuple: {:?} and Values: {}, {}, {}",
        tup, first, second, third
    );
    println!("First one: {}", tup.0);

    // --- ARRAY TYPE ---
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.
    // arrays have a fixed length.
    // Use array when you want to ensure you always have a fixed number of elements
    // An array isn’t as flexible as the vector type, though.
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    // If you’re unsure whether to use an array or a vector, chances are you should use a vector.
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let same_el_arr: [i32; 5] = [3; 5];
    println!("Arr: {:?}", arr);
    println!("Months: {:?}", months);
    println!("Same el arr: {:?}", same_el_arr);
    println!("First el: {}", arr[0]);
}
