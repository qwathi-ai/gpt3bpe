use std::collections::HashMap;

pub (crate) struct Grammar {
	lexicon: HashMap<String, Vec<Vec<String>>>
}

impl Grammar {
	pub fn new () -> Self {
		Self {lexicon: HashMap::new()}
	}
	pub fn get_right_hand_side(&self, root: &str) -> Vec<Vec<String>> {
		self.lexicon.get(root).unwrap().to_vec()
	}
}