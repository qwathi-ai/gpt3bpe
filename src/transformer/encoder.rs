use std::collections::HashMap;
use std::collections::HashSet;

type Pair = (String, String);
fn text_to_pairs(clone: &Vec<String>) -> HashSet<Pair> {
    let mut pairs: HashSet<Pair> = HashSet::new();
    for pair in clone.windows(2) {
        pairs.insert((pair[0].to_owned(), pair[1].to_owned()));
    }
    pairs
}

fn u16_to_string(c: &u16) -> String {
    String::from_utf16(&vec![*c]).unwrap()
}

fn list_unicodes() -> Vec<u16> {
    vec![
        33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78,
        79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
        119, 120, 121, 122, 123, 124, 125, 126, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170,
        171, 172, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
        190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
        208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
        226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243,
        244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ]
}

pub fn text_to_unicodes(text: &str, unicodes: &HashMap<u16, String>) -> Vec<String> {
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

        unicode
    };

    text.chars()
        .flat_map(|c| String::from(c).into_bytes())
        .map(|byte: u8| bytes_to_unicode(byte))
        .collect::<Vec<String>>()
}

fn bpe(unicodes: Vec<String>, vocab: &HashMap<Pair, usize>) -> Vec<String> {
    let mut bpe_tokens: Vec<String> = vec![];
    let mut pairs: HashSet<Pair> = text_to_pairs(&unicodes);
    if pairs.is_empty() {
        return unicodes;
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
        let first: &String = &bigram.1 .0;
        let second: &String = &bigram.1 .1;
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
            pairs = text_to_pairs(&clone);
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
    pub unicodes: HashMap<u16, String>,
    deunicodes: HashMap<String, u16>,
    // Used to stored already computed results
    cache: HashMap<Vec<String>, Vec<usize>>,
}

impl GPTEncoder {
    pub fn new(vocab: HashMap<Pair, usize>, encoder: HashMap<String, usize>) -> Self {
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

        let y = y
            .iter()
            .map(|x: &u16| u16_to_string(x))
            .collect::<Vec<String>>();
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

    pub fn encode(&mut self, tokens: &Vec<&str>) -> Vec<usize> {
        let mut bpe_encoding: Vec<usize> = vec![];

        for token in tokens {
            let unicodes = &text_to_unicodes(token, &self.unicodes);
            let bpencoding = match &self.cache.get(unicodes) {
                Some(bpcode) => bpcode.to_vec(),
                None => {
                    let unicodes = &bpe(unicodes.to_vec(), &self.vocab);
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
