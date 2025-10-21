fn main() {
    let name1 = String::from("dimas");

    // ownership moved to name2
    let name2 = name1;
    //  ownership now on name2
    println!("{}", name2);
}
