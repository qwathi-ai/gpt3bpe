mod error;

use amile::encode;
use argh::FromArgs;

/// Reach new heights.
#[derive(Debug, FromArgs)]
struct Encode {
    /// string input.
    #[argh(option, short = 'i')]
    input: String,
}

fn main() {
    let arguments: Encode = argh::from_env();
    #[cfg(debug_assertions)]
    println!("[DEBUG][INPUT]: {:?}", arguments.input);
    let e = encode(arguments.input.as_bytes()).unwrap();
    println!("[INFO][ENCODE]: {:?}", e);
}
