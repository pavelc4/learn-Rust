fn main() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    // let array: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];

    println!("{}", a);
    println!("{}", b);

    array[0] = 20;
    array[1] = 10;

    let a_baru = array[0];
    let b_baru = array[1];

    println!("{:?}", a_baru);
    println!("{:?}", b_baru);

    let langth: usize = array.len();
    println!("{}", langth);
}
