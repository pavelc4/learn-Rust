fn main() {
    // diluar scope
    let nama = "dimas";

    {
        let nama_sope = "dimas di dalam scope";
        println!("inside scope{}: ", nama_sope);
    }
    println!(" variabel di luar cope  nama_scope{} :", nama);
    println!(" variabel di dalam iner scope :{}", nama_sope);
}
