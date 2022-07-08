// pub mod encoder;
pub mod reader;
// pub mod factory;
pub mod grammar;
pub mod chart;
pub mod state;


// fn main() {
//     let vocab = factory::vocab();
//     let gpt = factory::gpt();
//     let mut encoder = encoder::GPTEncoder::new(vocab.to_owned(), gpt.to_owned());

//     let stream = reader::StreamReader::open("test_gpt.txt").expect("Could not open file!");
//     for buffer in stream {
//         let text = buffer.unwrap();
//         println!("text => {:?}", &text);
//         let encoded = encoder.encode(&text);
//         println!("encoded => {:?}", encoded);
//         let decoded = encoder.decode(encoded);
//         // println!("decoded => {:?}", encoder.decode(encoded));
//         // let g = parse(text.chars().collect(), grammar, char::from_str("S").unwrap());
//         // println!("grammar => {:#?}", g )
//     };
// }

pub fn parse(tokens: Vec<&str>, grammar: grammar::Grammar, root: String) -> chart::Chart {

	let chart = chart::Chart::new(tokens);
	let root_rhs = grammar.get_right_hand_side(&root);
	for rule in root_rhs {
		let initial_state: state::State = state::State::new(&root, rule, 0, 0, 0);
		let changed = chart.add(0, initial_state);
	}
	for token in tokens{
        let mut changed = true;
        while changed {
            changed = false;
            let mut j = 0;
            while j < chart.count(i) {
                let state = chart.get(i);
                if !state.complete {
                    if state.non_terminal(grammar) {
                        state.predict(&grammar, chart);
                        changed |= state.changed;
                    } else {
                        state.scan(&grammar, chart, token);
                        changed |= state.changed;
                    }
                } else {
                    state.complete(&grammar, chart);
                    changed |= state.changed;
                }
                j += 1;
            }
        }
        println!("chart => : {:?}", chart);
    }
    chart
}  

fn tokenize(text: String) -> Vec<&'static str> {
    vec![]
}

fn main() {
    let grammar = grammar::Grammar::new();
    let stream = reader::StreamReader::open("test_nlp.txt").expect("Could not open file!");
    for buffer in stream {

        let text = buffer.unwrap();
        println!("text => {:?}", text);
        let tokens = tokenize(text.to_string());
        println!("tokens => {:?}", tokens);
        parse(tokens, grammar, "S".to_string());
    };
}
