fn main() {
    let value = 10;
    let result: &str;

    if value > 8 {
        result = "good value";
    } else if value > 7 {
        result = "ok value";
    } else if value > 5 {
        result = "not bad value ";
    } else {
        result = "bad value";
    }

    println!("{}", result);
}
