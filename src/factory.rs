use std::collections::HashMap;
use serde_json::from_str;

use crate::{grammar, reader};

pub fn vocab() -> HashMap<(String, String), usize> {
    let mut result = HashMap::new();
    let data: &str = &std::fs::read_to_string("vocab.bpe").expect("Unable to read bpe merges.");
	for (i, line) in data.lines().enumerate() {
        let bigram: Vec<String> = line.split_whitespace().map(|s|String::from(s)).collect::<Vec<_>>(); 
        result.insert((bigram[0].to_string(), bigram[1].to_string()), i);
	};
	result
}

pub fn gpt() -> HashMap<String, usize> {
    let data: &str = &std::fs::read_to_string("encoder.json").expect("Unable to read encoding.");
	let result: HashMap<String, usize> = from_str(data).unwrap();
    result
}

pub fn get_grammar()-> grammar::Grammar {
	let mut grammar = grammar::Grammar::new();
    let stream = reader::StreamReader::open("grammar.txt").expect("Could not open file!");
    for buffer in stream {

        let text = buffer.unwrap();
        println!("text => {:?}", &text);
        let (key, old, new) = grammar.add(&text.as_str());
        println!("key => {:?} \nold => {:?}\nnew => {:?}", key, old, new);
    };
	grammar
}