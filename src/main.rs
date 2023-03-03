mod ceasar_cipher;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // type_of_encryption: String,
    #[arg(short, long)]
    encode_decode: char,

    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    shift: u8,
}

fn main() {
    let args = Args::parse();
    
    let enc_dec = match args.encode_decode {
        'e' => ceasar_cipher::encode(args.input.clone(), args.shift).unwrap(),
        'd' => ceasar_cipher::decode(args.input.clone(), args.shift).unwrap(),
        _ => panic!("must choose e(encode)/d(decode)")
    };
    println!("origin -> {}", args.input);
    println!("{} -> {}", args.encode_decode, enc_dec);
}
