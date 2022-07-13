use std::rc::Rc;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn tokenize(text: &Rc<String>) -> Vec<&str> {
    let re: Regex = Regex::new(
        r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+",
    )
    .unwrap();
    let mut tokens: Vec<&str> = vec![];
    for mat in re.find_iter(text) {
        tokens.push(mat.as_str())
    }
    tokens
}

type Pair = (String, String);
fn string_to_pairs(clone: &Vec<String>) -> HashSet<Pair> {
    let mut pairs: HashSet<Pair> = HashSet::new();
    for pair in clone.windows(2) {
        pairs.insert((pair[0].to_owned(), pair[1].to_owned()));
    }
    pairs 
}

fn string_to_utf16(c: &str) -> u16 {
    c.encode_utf16().collect::<Vec<u16>>()[0]
}

fn range_between(x: u16, y: u16) -> Vec<u16> {
    (x..y).collect::<Vec<u16>>()
}

fn u16_to_string(c: &u16) -> String {
    String::from_utf16(&vec![*c]).unwrap()
}

fn list_unicodes() -> Vec<u16> {
    let mut bs: Vec<u16> = vec![];
    bs.extend(range_between(string_to_utf16("!"), string_to_utf16("~") + 1));
    bs.extend(range_between(string_to_utf16("¡"), string_to_utf16("¬") + 1));
    bs.extend(range_between(string_to_utf16("®"), string_to_utf16("ÿ") + 1));
    bs
}

fn token_to_unicodes(token: &str, unicodes: &HashMap<u16, String>) -> Vec<String> {
    let bytes_to_unicode = |c: u8| -> String {

        let unicode = match unicodes.get(&(c as u16)) {
            Some(ch) => ch.to_string(),
            None => {
                println!(
                    "could not find unicode for {:?}. Using Fallback value of {:?}",
                    c,
                    char::from_u32(c.into()).unwrap()
                );
                char::from_u32(c.into()).unwrap().to_string()
            }
        };
        
        unicode.to_string()
    };
    
    token.chars()
    .flat_map(|c| String::from(c).into_bytes())
    .map(|byte: u8| bytes_to_unicode(byte))
    .collect::<Vec<String>>()
}

fn bpe(unicodes: &Vec<String>, vocab: &HashMap<Pair, usize>) -> Vec<String> {
    let mut bpe_tokens: Vec<String> = vec![];
    let mut pairs: HashSet<Pair> = string_to_pairs(unicodes);
    if pairs.is_empty() {
        return unicodes.to_vec()
    };

    let mut clone: Vec<String> = unicodes.clone();
    loop {
        let mut min_pairs: Vec<(&usize, &Pair)> = vec![];
        for pair in pairs.iter() {
            let rank: &usize = vocab.get(pair).unwrap_or(&usize::MAX);
            // println!("rank => {:?} | pair {:?}",rank, pair);
            min_pairs.push((rank, pair));
        }
        min_pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let bigram = &min_pairs[0];
        // println!("bigram => {:?}",bigram);

        if bigram.0 == &usize::MAX {
            break;
        };
        let first: &String = &bigram.1.0;
        let second: &String = &bigram.1.1;
        let mut new_word: Vec<String> = vec![];
        let mut i = 0;

        while i < clone.len() {
            let part = clone[i..].to_vec(); // ['h','e','l','l','o']
            let mut j = part.iter().position(|r| r == first).unwrap_or(usize::MAX);
            // println!("part => {:?} | i => {:?} | j => {:?} | first => {:?} | second => {:?}",part, i, j, first, second);
            if j == usize::MAX {
                new_word.extend(part);
                break;
            }
            j += i;
            new_word.extend(clone[i..j].to_vec());
            // println!("new word => {:?}",new_word);
            i = j;

            // println!("test case => {:?}",&clone[i] == first && &i < &(&clone.len() - 1) && &clone[i + 1] == second);
            if &clone[i] == first && &i < &(&clone.len() - 1) && &clone[i + 1] == second {
                let new_part = format!("{}{}", first, second);
                new_word.push(new_part);
                // println!("true case => {:?}",new_word);
                i += 2;
            } else {
                let new_part = &clone[i];
                new_word.push(new_part.to_string());
                // println!("false case => {:?}",new_word);
                i += 1;
            }
        }
        // println!("new word => {:?}", new_word);

        clone = new_word;
        if clone.len() == 1 {
            break;
        } else {
            pairs = string_to_pairs(&clone);
        };
    }
    bpe_tokens.extend(clone.to_vec());
    bpe_tokens
}

pub(crate) struct GPTEncoder {
    pub vocab: HashMap<Pair, usize>,
    pub encoder: HashMap<String, usize>,
    pub decoder: HashMap<usize, String>,
    // Used to keep local reference of
    unicodes: HashMap<u16, String>,
    deunicodes: HashMap<String, u16>,
    // Used to stored already computed results
    cache: HashMap<Vec<String>, Vec<usize>>,
}

impl GPTEncoder {
    pub fn new( vocab: HashMap<Pair, usize>, encoder: HashMap<String, usize> ) -> Self {
        let mut x = list_unicodes();
        let mut y: Vec<u16> = x.clone();
        let mut n: u16 = 0;

        for i in 0..=256 {
            if !x.contains(&i) {
                x.push(i);
                y.push(256 + n);
                n += 1;
            }
        }

        let y = y.iter().map(|x: &u16| u16_to_string(x)).collect::<Vec<String>>();
        let mut unicodes: HashMap<u16, String> = HashMap::new();
        let mut deunicodes: HashMap<String, u16> = HashMap::new();
        for (i, b) in x.iter().enumerate() {
            unicodes.insert(*b, y[i].to_owned());
            deunicodes.insert(y[i].to_owned(), *b);
        }
        let mut decoder = HashMap::new();
        for (k, v) in encoder.iter() {
            decoder.insert(v.clone(), k.clone());
        }
        
        let cache = HashMap::new();
        Self {
            encoder,
            decoder,
            vocab,
            unicodes,
            deunicodes,
            cache: cache.to_owned(),
        }
    }

    pub fn encode(&mut self, buffer: &Rc<String>) -> Vec<usize> {
        let mut bpe_encoding: Vec<usize> = vec![];
        
        let tokens = tokenize(&buffer);
        for token in tokens {
            let unicodes = &token_to_unicodes(token, &self.unicodes);
            let bpencoding = match &self.cache.get(unicodes) {
                Some(bpcode) => bpcode.to_vec(),
                None => {
                    let unicodes = &bpe(&unicodes.to_vec(), &self.vocab);
                    let bpecodes = unicodes
                        .iter()
                        .map(|c| *self.encoder.get(c).unwrap_or(&usize::MAX))
                        .collect::<Vec<_>>()
                        .to_vec();

                    self.cache.insert(unicodes.to_vec(), bpecodes.to_vec());
                    bpecodes
                }
            };
            // println!("token => {:?}  | unicode => {:?} | bpe => {:?}", token, unicodes, bpencoding);
            bpe_encoding.extend(bpencoding);
        }
        bpe_encoding
    }

    pub fn decode(&self, buffer: Vec<usize>) -> Vec<String> {
        let mut text: Vec<String> = vec![];
        let words = buffer.iter().map(|token| self.decoder.get(token).unwrap());
        for word in words {
            let t = word
                .chars()
                .map(|c: char| {
                    let d = self.deunicodes.get(&c.to_string()).unwrap();
                    String::from_utf16(&[*d]).unwrap()
                })
                .collect::<String>();

            text.push(t.trim().to_string())
        }
        text
    }
}
