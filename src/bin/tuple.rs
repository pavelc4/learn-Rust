fn main() {
    //let  data: (i32, f64,bool) = (10, 10.5,ture);
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    /* let a = data.0;
     let b = data.1;
     let c = data.2;
     let name = data.3;
    */

    let (a, b, c) = data;
    println!("{}{}{}", a, b, c);

    data.0 = 20;
    data.1 = 30.12;
    data.2 = false;

    println!("{:?}", data)
}
