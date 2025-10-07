/*
 * TABEL CONTOH OPERATOR ARITMATIKA RUST
 * 
 * +----------+---------------------+-------+
 * | Operator | Contoh Code         | Hasil |
 * +----------+---------------------+-------+
 * | +        | let hasil = 7 + 3;  | 10    |
 * | -        | let hasil = 10 - 4; | 6     |
 * | *        | let hasil = 5 * 6;  | 30    |
 * | /        | let hasil = 15 / 4; | 3     |
 * | %        | let hasil = 15 % 4; | 3     |
 * +----------+---------------------+-------+
 */


fn main() {
    let  a  = 11;
    let b = 1;
    
    let tambah = a +b ;
    println!("hail penjumlahan :{}",tambah);


    let  kali  = a + b;
    println!("hail perkalian : {}",kali);


    let bagi = a / b; 
    println!("hasil pembagian : {}", bagi);

    let moddulo = a % b;
    println!("Hasil sisa Bagi{}",moddulo);
    
}