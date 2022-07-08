use std::collections::HashMap;
use serde_json::from_str;

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