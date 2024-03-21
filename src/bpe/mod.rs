mod unit;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::mem::swap;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    #[derive(Debug)]
    static ref ENCODER: BTreeMap<String, i32>  ={
        let mut encoder = std::collections::BTreeMap::new();
        let file = std::fs::File::open("src/bpe/lookup.jsonl").expect("Unable to open file lookup.jsonl");
        let file = std::io::BufReader::new(file);

        for (_idx, line) in std::io::BufRead::lines(file).enumerate() {
            let line = line.unwrap();
            let data: BTreeMap<String,i32>  = serde_json::from_str(&line).unwrap();
            encoder.extend(data)
        }
        encoder
    };

    static ref DECODER: BTreeMap<i32, String> = {
        let mut decode = std::collections::BTreeMap::new();
        for (key, value) in ENCODER.iter() {
            if key.split_whitespace().collect::<Vec<&str>>().len() == 1 {
                decode.insert(value.to_owned(), key.to_owned());
            }
        }
        decode
    };
}

fn encode_key(key: &str) -> Option<i32> {
    match ENCODER.get(&key.to_string()) {
        Some(encoding) => Some(encoding.to_owned()),
        None => None,
    }
}

fn encode_pair(pair: &[String; 2]) -> Option<i32> {
    let key = String::from(format!("{} {}", pair[0], pair[1]));
    match ENCODER.get(&key.to_string()) {
        Some(encoding) => Some(encoding.to_owned()),
        None => None,
    }
}

fn decode_value(value: &i32) -> Option<String> {
    match DECODER.get(&value) {
        Some(encoding) => Some(encoding.to_owned()),
        None => None,
    }
}

fn split(part: &String) -> Vec<String> {
    UnicodeSegmentation::graphemes(part.as_str(), true)
        .map(|g| g.to_string())
        .collect()
}

fn can_join(pair: &[String; 2], comparison: &[String; 2]) -> bool {
    let pair_left = split(&pair[0]);
    let pair_right = split(&pair[1]);
    let comparison_left = split(&comparison[0]);
    let comparison_right = split(&comparison[1]);

    pair_left.last() == comparison_left.last() && pair_right.first() == comparison_right.first()
}

fn join(pair: &[String; 2]) -> String {
    pair.to_vec().iter().map(|part| part.to_owned()).collect()
}

fn zip(pairs: &Vec<[String; 2]>) -> Vec<String> {
    let mut parts = vec![];

    if pairs.len() == 1 {
        let part = join(&pairs[0]).to_owned();
        parts.push(part);
        return parts.to_vec();
    };

    let mut cursor = pairs.iter().peekable();
    while let Some(part) = cursor.next() {
        parts.push(part[0].to_owned());
        if cursor.peek().is_none() {
            parts.push(part[1].to_owned());
        };
    }
    parts.to_vec()
}

pub fn unzip(parts: &Vec<String>) -> Vec<[String; 2]> {
    parts
        .windows(2)
        .map(|pair| [pair[0].to_owned(), pair[1].to_owned()])
        .collect()
}

fn merge(grapheme: &Vec<String>, pair: &[String; 2]) -> Vec<String> {
    let pairs = unzip(&grapheme);
    let mut binding = pairs.to_vec();
    let mut resolver = grapheme.to_vec();
    let mut cursor = pairs.iter().enumerate().peekable();

    'merge: while let Some((current_idx, current)) = cursor.next() {
        if let Some((next_idx, next)) = cursor.peek() {
            if can_join(&pair, current) {
                let left = join(current);
                swap(&mut binding[current_idx], &mut [left, next[1].to_owned()]);
                binding.remove(*next_idx);
                resolver = zip(&binding);
                break 'merge;
            };

            if can_join(&pair, next) && next_idx == &(binding.len() - 1) {
                let right = join(next);
                swap(
                    &mut binding[current_idx],
                    &mut [current[0].to_owned(), right],
                );
                binding.pop();
                resolver = zip(&binding);
                break 'merge;
            };
        }
        if can_join(&pair, current) && pairs.len() < 2 {
            resolver = vec![join(current)];
        };
    }
    resolver.to_vec()
}

pub fn encode(grapheme: &Vec<String>) -> Vec<i32> {
    let mut encoding = vec![];
    let mut pairs = unzip(&grapheme);

    if pairs.is_empty() {
        for key in grapheme {
            match encode_key(&key) {
                Some(value) => encoding.push(value),
                None => {
                    panic!("ERROR: Encoding value for {:?} not found!", &key);
                }
            }
        }
        return encoding;
    }

    let mut cache = HashSet::new();
    let mut bigrams = vec![];
    let mut graph = grapheme.to_vec();

    'pair: loop {
        for pair in pairs.to_vec() {
            if !cache.contains(&pair) {
                if let Some(rank) = encode_pair(&pair) {
                    bigrams.push((rank, [pair[0].to_owned(), pair[1].to_owned()]));
                };
                cache.insert(pair);
            };
        }

        if bigrams.is_empty() {
            break 'pair;
        }

        bigrams.sort_by(|a, b| a.0.cmp(&b.0));

        'bigram: while let Some((_rank, bigram)) = bigrams.pop() {
            let graph_clone = merge(&graph, &bigram);

            let mut encodings_clone = vec![];
            for key in graph_clone.iter() {
                if let Some(value) = encode_key(&key) {
                    encodings_clone.push(value);
                } else {
                    continue 'bigram;
                }
            }
            graph = graph_clone;
            encoding = encodings_clone;
            pairs = unzip(&graph);
            println!(
                "grapheme:    {:?}\npairs:    {:?}\nbigrams:  {:?}\nencoding:   {:?}\n",
                graph, pairs, bigrams, encoding
            );
            if pairs.is_empty() {
                break 'pair;
            };
        }

        if graph.len() <= 1 {
            break 'pair;
        }
    }
    encoding.to_vec()
}

pub fn decode(encoding: &Vec<i32>) -> Vec<String> {
    let mut decoding = vec![];
    for value in encoding.iter() {
        if let Some(decoded) = decode_value(value) {
            decoding.push(decoded.to_owned());
        }
    }
    decoding
}
