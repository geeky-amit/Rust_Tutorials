fn main() {
    // A scalar type represents a single value. Rust has four primary scalar types:
    // integers, floating-point numbers, Booleans, and characters.

    // This type declaration indicates that the value it’s associated with should be an unsigned integer
    // (signed integer types start with i instead of u) that takes up 32 bits of space

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16 	u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive,
    // where n is the number of bits that variant uses. So an i8 can store numbers
    // from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers
    // from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

    // Additionally, the isize and usize types depend on the architecture of the computer
    // your program is running on, which is denoted in the table as “arch”: 64 bits if you’re
    // on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // Floating Numbers

    // Floating-point numbers are represented according to the IEEE-754 standard.
    // The f32 type is a single-precision float, and f64 has double precision.

    // let x: f64 = 2.0; // f64

    // let y: f32 = 3.0; // f32

    //let x = 50 % 3;

    // Boolean Values

    // let t = true;

    // let f: bool = false; // with explicit type annotation

    // The Character Type

    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

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

    println!("The value of x is:");
}
