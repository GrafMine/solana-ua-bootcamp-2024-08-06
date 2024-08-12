use bs58;
use std::io::{self, BufRead};

fn main() {
    println!("1. Convert from Base58 to wallet format");
    println!("2. Convert from wallet format to Base58");
    println!("Choose an option (1 or 2):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => base58_to_wallet(),
        "2" => wallet_to_base58(),
        _ => println!("Invalid choice"),
    }
}

fn base58_to_wallet() {
    println!("Enter your Base58 key:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

fn wallet_to_base58() {
    let wallet: Vec<u8> = vec![105, 84, 232, 55, 3, 206, 12, 40, 146, 206, 192, 212, 5, 103, 194, 187, 127, 222, 188, 231, 48, 106, 197, 131, 163, 55, 132, 61, 128, 221, 92, 208, 71, 49, 48, 175, 14, 4, 74, 76, 157, 240, 80, 2, 88, 185, 231, 122, 141, 142, 114, 232, 185, 228, 109, 159, 28, 159, 48, 99, 5, 122, 146, 46];
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}
