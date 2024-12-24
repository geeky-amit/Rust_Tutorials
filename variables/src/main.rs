fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // Constants aren’t just immutable by default—they’re always immutable.
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    // we can not reasign or change the value

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing

    // let x = 5;
    // println!("The value of first x: {x}");

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");
    //1st
    // let spaces = "   ";
    // println!("{spaces}");
    // let spaces = spaces.len();
    // println!(" {spaces}")

    // 2nd code will throw an error
    // The first spaces variable is a string type and the second spaces variable is a number type.
    // Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num;
    // instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here,
    // we’ll get a compile-time error:

    // let mut spaces = "   ";
    // println!("{spaces}");
    // spaces = spaces.len();
    // println!(" {spaces}")
}
