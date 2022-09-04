use std::collections::HashMap;
use std::collections::HashSet;

pub type Rule = HashSet<Vec<String>>;
#[derive()]
pub struct Grammar {
	lexicon: HashMap<String, Rule>
}

fn string_to_rule (rule: &str) -> (String, Rule) {
	let tuple = rule.split("->").collect::<Vec<&str>>();
	let lhs = tuple[0].trim();
	let rhss = tuple[1].trim();

	let mut rhs:HashSet<Vec<String>> = HashSet::new();
	rhss.split("|")
	.for_each(|part| {
		let part = part.split(" ")
		.filter(|s|*s != "")
		.map(|s| s.trim().to_string())
		.collect::<Vec<String>>();

		rhs.insert(part);
	});
	(lhs.to_string(), rhs)
}


impl Grammar{
	pub fn new () -> Self {
		Self {lexicon: HashMap::new()}
	}

	pub fn add(&mut self, rule: &str) -> (String, Rule, Rule) {
		let (key, mut set) = string_to_rule(&*rule.clone());
		let hs: Rule = HashSet::new();
		let old = self.lexicon.get(&key).unwrap_or_else(|| { &hs });
		for part in old {
			set.insert(part.to_vec());
		};
		let old = self.lexicon.insert(key.clone(), set.clone()).unwrap_or_else(|| { hs });
		(key, old, set)
	}

	pub fn rhs(&self, root: &str) -> Option<&Rule> {
		self.lexicon.get(root)
	}
}