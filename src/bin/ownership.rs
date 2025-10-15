fn main() {
    //a tidak bisa di akses di sinih karena belum di dekklarasian
    let a = 10; // a bisa di akses karena sudah di deklarasikan 

    {
        // b tidak bisa diakses di sinih karena belum di dekalrasikan
        let b = 20; // b bisa di akalses dari sinih karena sudah di dekalarasikan 
        println!("{}", b);
    } // b sudah di hapus dari scope karena data sudah di clear oleh rust itu sendiri 
    println!("{}", a);
} // sccope a selesai a akan di hapus dan tidak bisa di akes 
