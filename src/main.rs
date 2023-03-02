mod ceasar_cipher;

fn main() {
    let x = String::from("Hello There");
    println!("{}", x);
    let x = ceasar_cipher::encode(x, 8).unwrap();
    println!("{}", x);
    // let x = ceasar_cipher::decode(x, 8).unwrap();
    // println!("{}", x);
}
