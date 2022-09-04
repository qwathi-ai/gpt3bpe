use std::rc::Rc;
use regex::Regex;
use crate::chart;
use crate::grammar;
use crate::state;

pub fn tokenize(text: &Rc<String>) -> Vec<&str> {
    let re: Regex = Regex::new(
        r"(?u)'s|'t|'re|'ve|'m|'l l|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(\S)|\s+",
    )
    .unwrap();
    let mut tokens: Vec<&str> = vec![];
    for mat in re.find_iter(text) {
        tokens.push(mat.as_str())
    }
    tokens
}

pub fn parse(tokens: Vec<&str>, grammar: &grammar::Grammar, root: String) -> chart::Chart {
    let chart = chart::Chart::new(&tokens);
    // println!("chart => {:?}", chart);
    let rhs = grammar.rhs(&root);
    // println!("RHS => {:?}", rhs);
    for rule in rhs {
        let initial_state: state::State = state::State::new(&root, rule, 0, 0, 0);
        println!("State => {:?}", initial_state);
        let changed = chart.add(0, initial_state);
        println!("Chart => {:?}", chart);
    }
    // for (i, token) in tokens.iter().enumerate() {
    //     let mut changed = true;
    //     while changed {
    //         changed = false;
    //         let mut j = 0;
    //         while j < chart.count(i) {
    //             let state = chart.get(i);
    //             if !state.complete {
    //                 if state.non_terminal(grammar) {
    //                     state.predict(grammar, &chart);
    //                     changed |= state.changed;
    //                 } else {
    //                     state.scan(grammar, &chart, token);
    //                     changed |= state.changed;
    //                 }
    //             } else {
    //                 state.complete(grammar, &chart);
    //                 changed |= state.changed;
    //             }
    //             j += 1;
    //         }
    //     }
    //     println!("chart => : {:?}", chart);
    // }
    chart
}
