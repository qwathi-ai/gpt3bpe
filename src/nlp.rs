use std::rc::Rc;

use crate::factory;
use crate::grammar;
use crate::chart;
use crate::reader;
use crate::state;

pub fn parse(tokens: Vec<String>, grammar: &grammar::Grammar, root: String) -> chart::Chart {
	let chart = chart::Chart::new(&tokens);
	let rhs = grammar.rhs(&root);
	for rule in rhs {
		let initial_state: state::State = state::State::new(&root, rule, 0, 0, 0);
		let changed = chart.add(0, initial_state);
	}
	for (i, token) in tokens.iter().enumerate(){
        let mut changed = true;
        while changed {
            changed = false;
            let mut j = 0;
            while j < chart.count(i) {
                let state = chart.get(i);
                if !state.complete {
                    if state.non_terminal(grammar) {
                        state.predict(grammar, &chart);
                        changed |= state.changed;
                    } else {
                        state.scan(grammar, &chart, token);
                        changed |= state.changed;
                    }
                } else {
                    state.complete(grammar, &chart);
                    changed |= state.changed;
                }
                j += 1;
            }
        }
        println!("chart => : {:?}", chart);
    }
    chart
}  
