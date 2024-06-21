
use std::env;
use amile::text_encode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let message = args[1].as_bytes();
    let encoded = text_encode(message);
    println!("[DEBUG]:  {:?}  -> {:?}", args[1], encoded);
}