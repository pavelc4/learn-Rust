fn main() {
    let value = 10;
    let result = if value > 9 {
        "good"
    } else if value > 8 {
        "not bad"
    } else if value < 7 {
        "ok"
    } else {
        "bad"
    };

    println!("{}", result)
}
