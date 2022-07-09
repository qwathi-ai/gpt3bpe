use crate::grammar;
use crate::chart;

pub struct State {
	pub complete: bool,
	pub changed: bool
}

impl State {
	pub fn new (lhs: &str, rhs: Vec<String>, dot: usize, left: usize, right: usize) -> Self {
		Self {complete: false, changed: false}
	}
	pub fn non_terminal(&self, grammar: &grammar::Grammar) -> bool {
		false
	}
	pub fn predict(&self, grammar: &grammar::Grammar, chart: &chart::Chart) -> Option<chart::Chart> {
		None
	}
	pub fn scan(&self, grammar: &grammar::Grammar, chart: &chart::Chart, token: &str) -> Option<chart::Chart> {
		None
	}
	pub fn complete(&self, grammar: &grammar::Grammar, chart: &chart::Chart) -> Option<chart::Chart> {
		None
	}
}