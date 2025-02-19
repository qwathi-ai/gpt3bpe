use std::collections::HashSet;

type Rule<T> = HashSet<Vec<T>>;

static GRAMMAR: LazyLock<BTreeMap<u16, Rule<u8>>> = LazyLock::new(|| {
    // Create a dummy grammar for testing.
    let mut grammar = BTreeMap::new();
    // S -> NP VP
    grammar.insert()
    // VP -> VP PP | V NP | V
    // PP -> P NP
    // NP -> Det N | N


    let mut x = GPT_UNICODES.to_vec();
    let mut y: Vec<u16> = x.clone();
    let mut n: u16 = 0;
    for i in 0..=256 {
        if !x.contains(&i) {
            x.push(i);
            y.push(256 + n);
            n += 1;
        };
    }

    for (i, c) in x.iter().enumerate() {
        let decoded = String::from_utf16_lossy(&[y[i]]);
        unicodes.insert(*c, decoded.into_bytes());
    }
    unicodes
});
struct Grammar<T> {
    lexicon: HashMap<T, Rule<T>>
}

impl Grammar{
	pub fn add(&mut self, ) -> (String, Rule, Rule) {
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