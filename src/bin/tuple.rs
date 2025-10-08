fn main() {
    let data: (i32, f64,bool , &str) = (10 ,10.5, true, "dimas");
    println!("{:?}",data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    let name = data.3;

    println!("{}{}{}{}", a, b, c, name);
}
