use crate::state;

#[derive(Debug)]
pub (crate) struct Chart {}

impl Chart {
	pub fn new (tokens: Vec<&str>) -> Self {
		Self{}
	}

	pub fn add(&self, index: isize, state: state::State) -> bool {
		false
	}

	pub fn count(&self, index: usize) -> i32 {
		0
	}

	pub fn get(&self, index: usize) -> state::State {
		state::State::new("S", vec![],0,0,0)
	}
}