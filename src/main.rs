use std::collections::HashMap;
use serde_json::from_str;
pub mod encoder;
pub mod reader;

fn vocab() -> HashMap<[String;2], usize> {
    let mut result = HashMap::new();
    let data: &str = &std::fs::read_to_string("vocab.bpe").expect("Unable to read bpe merges.");
	for (i, line) in data.lines().enumerate() {
        let bigram: Vec<String> = line.split_whitespace().map(|s|String::from(s)).collect::<Vec<_>>(); 
        result.insert([bigram[0].to_string(), bigram[1].to_string()], i);
	};
	result
}

fn gpt() -> HashMap<String, usize> {
    let data: &str = &std::fs::read_to_string("encoder.json").expect("Unable to read encoding.");
	let result: HashMap<String, usize> = from_str(data).unwrap();
    result
}

fn main() {
    let vocab = vocab();
    let gpt = gpt();
    let mut encoder = encoder::GPTEncoder::new(Some(vocab.to_owned()), Some(gpt.to_owned()));

    let stream = reader::StreamReader::open("text.txt").expect("Could not open file!");
    for buffer in stream {
        let text = buffer.unwrap();
        println!("text => {:?}", &text);
        let encoded = encoder.encode(&text);
        println!("encoded => {:?}", encoded);
        println!("decoded => {:?}", encoder.decode(encoded));
        // let g = parse(text.chars().collect(), grammar, char::from_str("S").unwrap());
        // println!("grammar => {:#?}", g )
    };
}
