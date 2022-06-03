use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::rc::Rc;

pub fn tokenize(text: &Rc<String>) -> Vec<&str> {
	let re: Regex = Regex::new(r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+").unwrap();
	let mut tokens: Vec<&str> = vec![];
	for mat in re.find_iter(text) {
		tokens.push(mat.as_str())
	}
	tokens
}

fn ord(c: &str) -> u16 {
	c.encode_utf16().collect::<Vec<_>>()[0]
}
fn range(x: u16, y: u16) -> Vec<u16> {
	let mut elements: Vec<u16> = vec![];
	for e in x..y { elements.push(e) }
	elements
}
fn chr(c: &u16) -> String {
	String::from_utf16(&vec![*c]).unwrap()
}
fn get_pairs(word: &Vec<String>) -> HashSet<[String;2]> {
	let mut pairs: HashSet<[String;2]> = HashSet::new();
	let mut _prev: String = "".to_string();
	for (i, _char) in word.iter().enumerate() {
		if i > 0 {
			pairs.insert([_prev.to_string(), _char.to_string()]);
		}
		_prev = _char.to_string();
	}
	pairs
}

pub (crate) struct GPTEncoder {
	pub vocab: HashMap<[String;2], usize>,
	pub encoder: HashMap<String, usize>,
	pub decoder: HashMap<usize, String>,
	// Used to keep local reference of
	unicodes: HashMap<u16, String>,
	deunicodes: HashMap<String, u16>,
	// Used to stored already computed results
	cache: HashMap<Vec<String>, Vec<usize>>
}

impl GPTEncoder {
	pub fn new(vocab: Option<HashMap<[String;2], usize>>, encoder: Option<HashMap<String, usize>>) -> Self {
		let mut bs: Vec<u16> = vec![];
		bs.extend(range(ord("!"), ord("~") + 1));
		bs.extend(range(ord("¡"), ord("¬") + 1));
		bs.extend(range(ord("®"), ord("ÿ") + 1));
	
		let mut cs: Vec<u16> = bs.clone();
		let mut n: u16 = 0;
	
		for i in 0..=256 {
		  if !bs.contains(&i) {
			bs.push(i);
			cs.push(256 + n);
			n += 1;
		  }
		}
	
		let cs = cs.iter().map(|x: &u16| chr(x)).collect::<Vec<String>>();
		let mut unicodes: HashMap<u16,String> = HashMap::new();
		let mut deunicodes: HashMap<String,u16> = HashMap::new();
		for (i, b) in bs.iter().enumerate() {
			unicodes.insert(*b, cs[i].to_owned());
			deunicodes.insert(cs[i].to_owned(), *b);
		};
		let cache = HashMap::new();
		let encoder = encoder.unwrap_or(HashMap::new());
		let mut decoder = HashMap::new();
		for (k,v) in encoder.iter() {
			decoder.insert(v.clone(), k.clone());
		};
				
		Self { 
			encoder, 
			decoder,
			vocab: vocab.unwrap_or(HashMap::new()), 
			unicodes, 
			deunicodes,
			cache: cache.to_owned()
		}
	}

	fn token_to_unicodes(&self, token: &str) -> Vec<String> {
		let bytes_to_unicode = |c: u8| -> String { 
			let unicode = match &self.unicodes.get(&(c as u16)) {
				Some(ch) => { ch.to_string() },
				None => { 
					println!("could not find unicode for {:?} fallback {:?}", c, char::from_u32(c.into()).unwrap() ); 
					char::from_u32(c.into()).unwrap().to_string()
				}
			};
			unicode.to_string()
		};
		let bytes = token.chars().flat_map(|c| String::from(c).into_bytes());
		bytes.map(|byte: u8| bytes_to_unicode(byte)).collect::<Vec<String>>()
	}

	fn bpe(&self, unicodes: &Vec<String>) -> Vec<String> {
		let mut bpe_tokens: Vec<String> = vec![];
		let mut pairs: HashSet<[String;2]> = get_pairs(&unicodes);
		if pairs.is_empty() {
			return unicodes.to_vec()
		};
		
		let mut clone: Vec<String> = unicodes.clone();
		loop {
			let mut min_pairs: Vec<(&usize, &[String;2])> = vec![];
			for pair in pairs.iter() {
				let rank: &usize = self.vocab.get(pair).unwrap_or(&usize::MAX);
				// println!("rank => {:?} | pair {:?}",rank, pair);
				min_pairs.push((rank, pair));
			};
			min_pairs.sort_by(|a,b|  a.0.cmp(&b.0) );
			let bigram = &min_pairs[0];
			// println!("bigram => {:?}",bigram);

			if bigram.0 == &usize::MAX {
				break;
			};
			let first: &String = &bigram.1[0];
			let second: &String = &bigram.1[1];
			let mut new_word: Vec<String> = vec![];
			let mut i = 0;
		
			while i < clone.len() {
				let part = clone[i..].to_vec(); // ['h','e','l','l','o']
				let mut j = part.iter().position(|r| r == first).unwrap_or(usize::MAX);
				// println!("part => {:?} | i => {:?} | j => {:?} | first => {:?} | second => {:?}",part, i, j, first, second);
				if j == usize::MAX  {
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
					i +=1;
				}			
			};
			// println!("new word => {:?}", new_word);
	
			clone = new_word;
			if clone.len() == 1 {
				break;
			} else {
				pairs = get_pairs(&clone);
			};
		};
		bpe_tokens.extend(clone.to_vec());
		bpe_tokens
	}

	pub fn encode(&mut self, buffer: &Rc<String>) -> Vec<usize> {
		let mut bpe_encoding: Vec<usize> = vec![];
		let tokens = tokenize(&buffer);
		for token in tokens {
			let unicodes = &self.token_to_unicodes(token);
			let bpencoding = match &self.cache.get(unicodes) {
				Some(bpcode) => bpcode.to_vec(),
				None => {
					let unicodes = &self.bpe(&unicodes.to_vec());
					let bpecodes = unicodes.iter().map(
						|c| *self.encoder.get(c).unwrap_or(&usize::MAX)
					).collect::<Vec<_>>().to_vec();

					self.cache.insert(unicodes.to_vec(), bpecodes.to_vec());
					bpecodes
				}
			};
			// println!("token => {:?}  | unicode => {:?} | bpe => {:?}", token, unicodes, bpencoding);		
			bpe_encoding.extend(bpencoding);
		};
		bpe_encoding
	}

	pub fn decode(&self, buffer: Vec<usize>) -> Vec<String> {
		let mut text: Vec<String> = vec![];
		let words =  buffer.iter().map(|token| self.decoder.get(token).unwrap());
		for word in words {
			
			let t = word.chars().map(|c:char| {
				let d = self.deunicodes.get(&c.to_string()).unwrap();
				String::from_utf16(&[*d]).unwrap()
			}).collect::<String>();

			text.push(t.trim().to_string())
		};
		text
	}

}