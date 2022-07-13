use std::collections::HashMap;
use std::collections::HashSet;

pub struct Grammar {
	lexicon: HashMap<String, HashSet<Vec<String>>>
}

fn parse_rule (rule: &str) -> (String, HashSet<Vec<String>>){

	// let re: Regex = Regex::new(
    //     r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+",
    // )
    // .unwrap();
    // let mut tokens: Vec<&str> = vec![];
    // for mat in re.find_iter(text) {
    //     tokens.push(mat.as_str())
    // }
    // tokens
	("A".to_string(), HashSet::new())
}
impl Grammar {
	pub fn new () -> Self {
		Self {lexicon: HashMap::new()}
	}

	pub fn add(&mut self, rule: &str) -> (String, HashSet<Vec<String>>, HashSet<Vec<String>>) {
		let (key, mut set) = parse_rule(rule);
		let old = self.lexicon.get(&key).unwrap();
		for part in old {
			set.insert(part.to_vec());
		};
		let old = self.lexicon.insert(key.clone(), set.clone()).unwrap();
		(key, old, set)
	}

	pub fn rhs(&self, root: &str) -> &HashSet<Vec<String>> {
		self.lexicon.get(root).unwrap()
	}
}