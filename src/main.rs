use std::io::Result;
pub mod reader;
pub mod stack;

fn main() -> Result<()>{
    for line in reader::StreamReader::open("foo.txt")? {
        let text = line?;
        let tokens: Vec<String> = stack::tokenize(&text);
        println!("TOKENS => : {:?}", tokens);
        match stack::parse(tokens) {
            Err(e) => println!("{:?}", e),
            _ => println!("Done!")
        };
        // print!("TREE => : {:?}", tree);
    };
    Ok(())
}