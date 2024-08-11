mod unit;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    #[derive(Debug)]
    static ref ENCODER: BTreeMap<Vec<u8>, u16> ={
        let mut encoder = std::collections::BTreeMap::new();
        let file = std::fs::File::open("src/encoder/bytepairs.jsonl")
        .expect("[ERROR]: Unable to open file encoder/bytepairs.jsonl");
        let file = std::io::BufReader::new(file);

        for line in std::io::BufRead::lines(file) {
            let _line = line.unwrap();
            let mut data: BTreeMap<String,u16>  = serde_json::from_str(_line.as_str())
                .expect("[ERROR]: Unable to read file encoder/bytepairs.jsonl");
            while let Some((key, value)) = data.pop_first() {
                encoder.insert(key.into_bytes(), value);
            }
        };
        encoder
    };

    #[derive(Debug)]
    static ref DECODER: BTreeMap<u16, Vec<u8>> = {
        let mut decode = std::collections::BTreeMap::new();

        for (key, value) in ENCODER.iter() {
            let string: String = String::from_utf8(key.to_vec())
            .expect("[ERROR]: Unable to split file encoder/bytepairs.jsonl");

            if string.split_whitespace().count() == 1 {
                decode.insert(value.clone(), key.to_vec());
            };
        };
        decode
    };
}

fn segment(bytes: &[u8]) -> Result<Vec<String>, crate::error::Error> {
    let text = String::from_utf16le(bytes).unwrap();

    Ok(UnicodeSegmentation::graphemes(text.as_str(), true)
        .map(|segment| segment.to_string())
        .collect())
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct BytePair{
    pair: [Vec<u8>; 2],
}

impl BytePair{
    fn compatible(&self, other: &BytePair) -> bool {
        let pair_left = segment(&self.pair[0]).unwrap();
        let pair_right = segment(&self.pair[1]).unwrap();
        let comparison_left = segment(&other.pair[0]).unwrap();
        let comparison_right = segment(&other.pair[1]).unwrap();
        pair_right.first() == comparison_right.first() && pair_left.last() == comparison_left.last()
    }
}

impl std::ops::Add< &BytePair> for BytePair{
    type Output = BytePair;

    fn add(mut self, rhs: &BytePair) -> Self::Output {
        self.pair[0] = [self.pair[0].to_owned(), self.pair[1].to_owned()].concat();
        self.pair[1] = [rhs.pair[0].clone(), rhs.pair[1].clone()].concat();
        self
    }
}
#[derive(Clone, Debug)]
struct Encoder {
    pub grapheme: Vec<Vec<u8>>,
    cache: HashSet<[Vec<u8>; 2]>,
    bigrams: Vec<(u16, BytePair)>,
}

impl Encoder{
    fn new(grapheme: &Vec<Vec<u8>>) -> Self {
        Encoder {
            grapheme: grapheme.to_vec(),
            cache: HashSet::new(),
            bigrams: vec![],
        }
    }
    fn encoding(&self) -> Option<Vec<Vec<u16>>> {
        let mut encoding = vec![];
        for key in self.grapheme.iter() {
            if let Some(value) = ENCODER.get(key) {
                encoding.push(vec![*value])
            }
        }
    
        match self.grapheme.len() == encoding.len() {
            true => Some(encoding),
            false => None,
        }
    }
    fn update(mut self) -> Encoder {
        let binding = self.grapheme.clone().clone();

        while let Some([left, right]) = binding.windows(2).next() {
            let pair = [left.clone(), right.clone()];

            if !self.cache.contains(&pair) {
                if let Some(rank) = ENCODER.get(&pair.clone().concat()) {
                    self.bigrams.push((
                        *rank,
                        BytePair {
                            pair: [left.to_vec(), right.to_vec()],
                        },
                    ))
                }
                self.cache.insert(pair);
            }
        }

        Self { grapheme: binding, cache: self.cache, bigrams: self.bigrams }
    }
}

fn zip(bigrams: &Vec<(u16, BytePair)>) -> Vec<Vec<u8>> {
    let mut cursor = bigrams
        .iter()
        .map(|gram| -> BytePair { gram.1.clone() })
        .peekable();

    let mut parts = vec![];
    while let Some(byte) = cursor.next() {
        parts.push(byte.pair[0].clone());
        if cursor.peek().is_none() {
            parts.push(byte.pair[1].clone());
        }
    }
    parts.to_vec()
}

impl std::ops::Add<&BytePair> for Encoder {
    type Output = Encoder;

    fn add(mut self, pair: &BytePair) -> Self::Output {
        let last = self.bigrams.len();
        let mut cursor = self.bigrams.iter_mut().enumerate().peekable();

        while let Some((index, current)) = cursor.next() {
            if let Some((_, next)) = cursor.peek() {
                if next.1.compatible(&pair) && (index + 1) == last {
                    let replace = next.1.to_owned().to_owned() + pair;
                    current.1 = BytePair {
                        pair: [current.1.pair[0].to_owned(), replace.pair[0].to_owned()],
                    };
                    self.bigrams.remove(index + 1);
                    self.grapheme = zip(&self.bigrams);
                    self = self.update();
                    break;
                };
                if current.1.compatible(&pair) {
                    let replace = current.1.to_owned() + pair;
                    current.1 = BytePair {
                        pair: [replace.pair[0].to_owned(), next.1.pair[1].to_owned()],
                    };
                    self.bigrams.remove(index + 1);
                    self.grapheme = zip(&self.bigrams);
                    self = self.update();
                    break;
                };
            }
        }
        self
    }
}

// Remains a public function
pub fn encode(grapheme: &Vec<Vec<u8>>) -> Result<Vec<Vec<u16>>, crate::error::Error> {
    let mut encoder = Encoder::new(&grapheme);
    let mut encoding = match encoder.encoding() {
        Some(enc) => enc,
        None => panic!("[ERROR]: Error encoding a character in {:?}", grapheme),
    };
    println!("[DEBUG]: {:?} -> {:?} -> {:?}", grapheme, encoder, encoding);

    'pairing: loop {
        encoder = encoder.update();
        if encoding.len() <= 1 || encoder.clone().bigrams.is_empty(){
            break 'pairing;
        };
        encoder.clone().bigrams.sort_by(|a, b| a.0.cmp(&b.0));

        while let Some((_rank, bytepair)) = encoder.clone().bigrams.pop() {
            let encoder = encoder.clone() + &bytepair;

            if encoder.grapheme.len() != encoding.len() {
                let mut _encoding = vec![];
                for key in encoder.grapheme.iter() {
                    if let Some(value) = ENCODER.get(key) {
                        _encoding.push(vec![*value])
                    }
                };
                if &encoder.grapheme.len() == &_encoding.len() {
                    encoding = _encoding;
                }
            }
        };
    }
    Ok(encoding)
}

// Remains a public function
pub fn decode(encoding: &Vec<u16>) -> Result<Vec<Vec<u8>>, crate::error::Error> {
    let mut decoding = vec![];
    for key in encoding {
        if let Some(value) = DECODER.get(key) {
            decoding.push(value.to_vec())
        }
    }

    match decoding.len() == encoding.len() {
        true => Ok(decoding.to_vec()),
        false => panic!(
            "[ERROR]: integer in grapheme {:?} could not be decoded.",
            encoding
        ),
    }
}
