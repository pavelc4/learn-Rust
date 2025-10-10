fn main() {
    let mut name: String = String::from("dimas");
    name.push_str("dwi");
    println!("{}", name);

    let ariyanto = name.replace("yayat", "dimas");
    println!("{}", ariyanto);
}
