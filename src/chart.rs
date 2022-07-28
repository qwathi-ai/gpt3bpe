use std::collections::HashSet;

use crate::state;

#[derive(Debug)]
pub struct Chart {
	
}

impl Chart {
	pub fn new (tokens: &Vec<String>) -> Self {
		Self{}
	}

	pub fn add(&self, index: isize, state: state::State) -> (isize, state::State, state::State) {
		let old = state.clone();
		(index, old, state)
	}

	pub fn count(&self, index: usize) -> i32 {
		0
	}

	pub fn append(&self, state: state::State)  -> bool {
		false
	}

	pub fn get(&self, index: usize) -> state::State {
		state::State::new("S", &HashSet::new(),0,0,0)
	}
}