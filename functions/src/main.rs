// fn main() {
//     println!("Hello, world!");
//     another_function();
// }

// fn another_function(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn another_function() {
//     let y = {
//         let x = 3;
//         x + 10
//     };

//     println!("The value of y is: {y}");
// }

// Function signatures:

// Rust is an expression-based language
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.

fn main() {
    let x = plus_one(10);
    // let x = 10;

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

// Functions with Return Values

// Functions can return values to the code that calls them. We don’t name return values,
// but we must declare their type after an arrow (->). In Rust, the return value of the
// function is synonymous with the value of the final expression in the block of the body of a function.
// You can return early from a function by using the return keyword and specifying a value,
// but most functions return the last expression implicitly

// But if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement,
// we’ll get an error:
