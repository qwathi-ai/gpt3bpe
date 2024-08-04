mod unit;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::mem::swap;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    #[derive(Debug)]
    static ref ENCODER: BTreeMap<Vec<u8>, u32> ={
        let mut encoder = std::collections::BTreeMap::new();
        let file = std::fs::File::open("src/encoder/text.jsonl").expect("[ERROR]: Unable to open file encoder/text.jsonl");
        let file = std::io::BufReader::new(file);

        for line in std::io::BufRead::lines(file) {
            let _line = line.unwrap();
            let mut data: Vec<(String,u32)>  = serde_json::from_str(_line.as_str()).expect("REASON");
            while let Some((key, value)) = data.pop() {
                encoder.insert(key.into_bytes(), value);
            };
        };
        encoder
    };

    #[derive(Debug)]
    static ref DECODER: BTreeMap<u32, Vec<u8>> = {
        let mut decode = std::collections::BTreeMap::new();

        for (key, value) in ENCODER.iter() {
            let string: String = String::from_utf8(key.to_vec()).expect("REASON");
            
            if string.split_whitespace().count() == 1 {
                decode.insert(value.clone(), key.to_vec());
            };
        };
        decode
    };
}

type BytePair<T> = [T;2];

trait ByteParing<T> {
    can_join()
}
struct Encoder<T> {
    grapheme: Vec<T>,
    cache: HashSet<BytePair<T>>,
    bigrams: Vec<BytePair<T>>
}



// impl Encoder {
//     // fn get_pair(pair: &[String; 2]) -> Option<&'static i32> {
//     //     let key = format!("{} {}", pair[0], pair[1]);
//     //     ENCODER.get(key.as_str())
//     // }
//     // 
//     // fn join(pair: &BytePair<T>) -> T {
//     //     pair.iter().map(|part| part.as_str()).collect()
//     // }
//     // // Addition
//     // fn merge(grapheme: &[String], pair: &[String; 2]) -> Vec<String> {
//     //     todo!()
//     // }
//     // fn from_pairs(pairs: &Vec<[T; 2]>) -> Vec<T> {
//     //     todo!()
//     // }
//     // fn can_join(pair: &[String; 2], comparison: &[String; 2]) -> bool {
//     //     todo!()
//     // }
// }


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

// this becomes an iterator 
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


// Remains a public function
pub fn encode<T>(bytes: Vec<T>) -> Result<Vec<u16>, crate::error::Error> {
    // let mut graph: Vec<String> = grapheme.iter().map(|p| p.to_string()).collect();

    let mut encoding = match {
        let mut encoding = vec![];
        for key in grapheme.iter() {
            if let Some(value) = ENCODER.get(key.as_str()) {
                encoding.push(*value)
            }
        }

        match &graph.len() == &encoding.len() {
            true => Some(encoding),
            false => None,
        }
    } {
        Some(value) => value,
        None => panic!(
            "[ERROR]: character in below: \n\n{:?}\n\n could not be encoded.",
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

// Remains a public function
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