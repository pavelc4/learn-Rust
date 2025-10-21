fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break; // stop the loop if counter value 10
        } else if counter % 2 == 0 {
            continue; // convert even number to integer
        }
        println!("counter : {}", counter)
    }
}
