mod unit;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::mem::swap;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    #[derive(Debug)]
    static ref TEXT_ENCODER: BTreeMap<String, i32>  ={
        let mut encoder = std::collections::BTreeMap::new();
        let file = std::fs::File::open("src/encoder/text.jsonl").expect("[ERROR]: Unable to open file encoder/text.jsonl");
        let file = std::io::BufReader::new(file);

        for (_idx, line) in std::io::BufRead::lines(file).enumerate() {
            let _line = line.unwrap();
            let data: BTreeMap<String,i32>  = serde_json::from_str(_line.as_str()).unwrap();
            encoder.extend(data)
        }
        encoder
    };

    static ref TEXT_DECODER: BTreeMap<i32, String> = {
        let mut decode: BTreeMap<i32, String> = std::collections::BTreeMap::new();
        for (key, value) in TEXT_ENCODER.iter() {
            if key.split_whitespace().count() == 1 {
                decode.insert(value.to_owned(), key.to_string());
            }
        }
        decode
    };
}

fn get_pair(pair: &[String; 2]) -> Option<&'static i32> {
    let key = format!("{} {}", pair[0], pair[1]);
    TEXT_ENCODER.get(key.as_str())
}

fn _encode(grapheme: &Vec<String>) -> Option<Vec<i32>> {
    let mut encoding = vec![];
    for key in grapheme {
        if let Some(value) = TEXT_ENCODER.get(key.as_str()) {
            encoding.push(*value)
        }
    }

    match grapheme.len() == encoding.len() {
        true => Some(encoding),
        false => None,
    }
}

pub fn decode(encoding: &Vec<i32>) -> Result<Vec<String>, crate::error::Error> {
    let mut decoding = vec![];
    for key in encoding {
        if let Some(value) = TEXT_DECODER.get(key) {
            decoding.push(value.to_string())
        }
    }

    match decoding.len() == encoding.len() {
        true => Ok(decoding),
        false => panic!(
            "[ERROR]: integer in grapheme {:?} could not be decoded.",
            encoding
        ),
    }
}

fn can_join(pair: &[String; 2], comparison: &[String; 2]) -> bool {
    let pair_left: Vec<&str> = {
        let part: &str = &pair[0];
        UnicodeSegmentation::graphemes(part, true).collect()
    };
    let pair_right: Vec<&str> = {
        let part: &str = &pair[1];
        UnicodeSegmentation::graphemes(part, true).collect()
    };
    let comparison_left: Vec<&str> = {
        let part: &str = &comparison[0];
        UnicodeSegmentation::graphemes(part, true).collect()
    };
    let comparison_right: Vec<&str> = {
        let part: &str = &comparison[1];
        UnicodeSegmentation::graphemes(part, true).collect()
    };

    pair_left.last() == comparison_left.last() && pair_right.first() == comparison_right.first()
}

fn join(pair: &[String; 2]) -> String {
    pair.iter().map(|part| part.as_str()).collect()
}

fn from_pairs(pairs: &Vec<[String; 2]>) -> Vec<String> {
    let mut parts = vec![];

    if pairs.len() == 1 {
        let part = join(&pairs[0]);
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

fn to_pairs(parts: &[String]) -> Vec<[String; 2]> {
    parts
        .windows(2)
        .map(|pair| [pair[0].to_owned(), pair[1].to_owned()])
        .collect()
}

fn merge(grapheme: &[String], pair: &[String; 2]) -> Vec<String> {
    let mut response: Vec<String> = grapheme.to_vec();
    let parts = to_pairs(&response);
    let mut binding = parts.to_vec();
    let mut cursor = parts.iter().enumerate().peekable();

    while let Some((index, current)) = cursor.next() {
        if let Some((_, next)) = cursor.peek() {
            if can_join(pair, current) {
                let left = join(current);
                swap(&mut binding[index], &mut [left, next[1].to_owned()]);
                binding.remove(index + 1);
                response = from_pairs(&binding);
                break;
            };
            if can_join(pair, next) && (index + 1) == binding.len() {
                let right = join(next);
                swap(&mut binding[index], &mut [current[0].to_owned(), right]);
                binding.remove(index + 1);
                response = from_pairs(&binding);
                break;
            };
        }
    }
    response
}

pub fn encode(grapheme: &[&str]) -> Result<Vec<i32>, crate::error::Error> {
    let mut graph: Vec<String> = grapheme.iter().map(|p| p.to_string()).collect();

    let mut encoding = match _encode(&graph) {
        Some(value) => value,
        None => panic!(
            "[ERROR]: character in grapheme {:?} could not be encoded.",
            graph
        ),
    };

    if to_pairs(&graph).is_empty() {
        return Ok(encoding);
    };

    let mut cache = HashSet::new();
    let mut bigrams = vec![];

    'pairing: loop {
        for [left, right] in to_pairs(&graph) {
            if !cache.contains(&[left.clone(), right.clone()]) {
                if let Some(rank) = get_pair(&[left.clone(), right.clone()]) {
                    bigrams.push((rank, [left.clone(), right.clone()]));
                };
                cache.insert([left, right]);
            };
        }

        if bigrams.is_empty() {
            break 'pairing;
        }

        bigrams.sort_by(|a, b| a.0.cmp(b.0));

        'encoding: while let Some((_rank, bigram)) = bigrams.pop() {
            let _graph = merge(&graph, &bigram);
            if _graph.len() != graph.len() {
                match _encode(&_graph) {
                    Some(value) => encoding = value,
                    None => continue 'encoding,
                };
                graph = _graph;
            }
        }

        if graph.len() <= 1 {
            break 'pairing;
        }
    }
    Ok(encoding.to_vec())
}
