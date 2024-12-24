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
   // let x = plus_one(5);
   let x = 10;

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
