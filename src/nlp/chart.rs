use std::collections::HashSet;

use crate::state;

#[derive(Debug)]
pub struct Chart {
	
}

impl Chart {
	pub fn new (tokens: &Vec<&str>) -> Self {
		Self{}
	}

	pub fn add(&self, index: isize, state: state::State) -> (isize, state::State, state::State) {
        newState.setId(this.currentId);
        // TODO: use HashSet + LinkedList
        var chartColumn = this.chart[position];
        for (var x in chartColumn) {
            var chartState = chartColumn[x];
            if (newState.equals(chartState)) {
            
                var changed = false; // This is needed for handling of epsilon (empty) productions
                
                changed = chartState.appendRefsToChidStates(newState.getRefsToChidStates());
                return changed;
            }
        }
        chartColumn.push(newState);
        this.idToState[this.currentId] = newState;
        this.currentId++;
        
        var changed = true; // This is needed for handling of epsilon (empty) productions
        return changed;
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