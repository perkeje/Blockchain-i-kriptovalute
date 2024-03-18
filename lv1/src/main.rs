use std::env;
mod base58;
mod rsa;
mod sha256;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <encode|decode|sha256|generate_ecdsa|sign> '<input>'");
        return;
    }

    match args[1].as_str() {
        "encode" => {
            if args.len() != 3 {
                println!("Usage: cargo run encode '<input>'");
                return;
            }
            let input = args[2].as_str();
            let encoded = base58::encode_base58(input.as_bytes());
            println!("Encoded: {}", encoded);
        }
        "decode" => {
            if args.len() != 3 {
                println!("Usage: cargo run decode '<input>'");
                return;
            }
            let input = &args[2];
            let decoded = base58::decode_base58(input);
            println!("Decoded: {}", String::from_utf8(decoded).unwrap());
        }
        "sha256" => {
            if args.len() != 3 {
                println!("Usage: cargo run sha256 '<input>'");
                return;
            }
            let input = &args[2];
            let hash = sha256::sha256(input);
            println!("SHA-256: {}", hash);
        }
        "rsa_generate" => {
            if args.len() != 2 {
                println!("Usage: cargo run ecdsa_generate");
                return;
            }
            let keys = rsa::generate_keys();
            println!("Private key:\n{}", keys.0);
            println!("Public key:\n{}", keys.1);
        }
        "encrypt" => {
            if args.len() != 4 {
                println!("Usage: cargo run encrypt <message> <public_key>");
                return;
            }
            let message = &args[2];
            let public_key = &args[3];
            let sign = rsa::sign_messge(message, public_key);
            println!("Sign:\n{:?}", sign);
        }
        "decrypt" => {
            if args.len() != 4 {
                println!("Usage: cargo run decrypt <message> <private_key>");
                return;
            }
            let message = &args[2];
            let private_key = &args[3];
            let message = rsa::decode_messge(message, private_key);
            println!("Message:\n{}", message);
        }
        _ => println!("Invalid operation."),
    }
}
