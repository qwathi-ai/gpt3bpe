use std::rc::Rc;

use crate::grammar;
use crate::chart;
use crate::reader;
use crate::state;
use unicode_segmentation::UnicodeSegmentation;

pub fn parse(tokens: Vec<&str>, grammar: &grammar::Grammar, root: String) -> chart::Chart {

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

fn tokenize(text: &str) -> Vec<&str> {
	UnicodeSegmentation::split_word_bounds(text)
	.collect::<Vec<&str>>()
}

fn get_grammar()-> grammar::Grammar {
	let mut grammar = grammar::Grammar::new();
    let stream = reader::StreamReader::open("grammar.txt").expect("Could not open file!");
    for buffer in stream {

        let text = buffer.unwrap();
        println!("text => {:?}", &text);
        let (key, old, new) = grammar.add(&text.as_str());
    };
	grammar
}

pub fn nlp() {
    let grammar = get_grammar();
    let stream = reader::StreamReader::open("nlp.txt").expect("Could not open file!");
    for buffer in stream {

        let text = buffer.unwrap();
        println!("text => {:?}", &text);
        let tokens = tokenize(&text.as_str());
        println!("tokens => {:?}", &tokens);
        parse((*tokens).to_vec(), &grammar, "S".to_string());
    };
}
