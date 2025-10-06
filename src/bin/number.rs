/*
 * TIPE DATA  DI RUST
 * 
 * INTEGER TYPES:
 * +----------+----------+-----------+
 * | Panjang  | Signed   | Unsigned  |
 * +----------+----------+-----------+
 * | 8-bit    | i8       | u8        |
 * | 16-bit   | i16      | u16       |
 * | 32-bit   | i32      | u32       |
 * | 64-bit   | i64      | u64       |
 * | 128-bit  | i128     | u128      |
 * +----------+----------+-----------+
 * 
 * FLOAT TYPES:
 * +----------+-----------+
 * | Panjang  | Tipe Data |
 * +----------+-----------+
 * | 32-bit   | f32       |
 * | 64-bit   | f64       |
 * +----------+-----------+
 * 
 * DEFAULT TYPES:
 * • Integer: i32
 * • Float:   f64
 */

fn main(){
    let a: i32  = 10;
    println!("{}", a);


    let  b: f32 = 10.5;
    println!("{}", b);

}