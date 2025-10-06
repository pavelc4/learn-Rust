/*
 * TYPE CONVERSION IN RUST
 * 
 * Rust can perform type conversion from smaller numeric types to larger ones, and vice versa.
 * 
 * However, it's important to note that when converting numeric types from larger to smaller sizes,
 * Integer Overflow can occur. This is a condition where the numeric value cannot be accommodated
 * by the target conversion type.
 * 
 * For example, if we have a number 1,000,000 as i32 and we convert it to i8,
 * Integer Overflow will occur because i8 cannot hold that value.
 * 
 * - To perform conversion, we can use the 'as' keyword
 */

fn main() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = a as i32;
    println!("{}", c);
}