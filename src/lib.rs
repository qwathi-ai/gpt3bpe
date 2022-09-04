// use std::collections::HashMap;
// pub mod chart;
// pub mod encoder;
// pub mod factory;
// pub mod grammar;
// pub mod nlp;
// pub mod reader;
// pub mod state;

// fn main() {
//     let vocab = factory::vocab();
//     let gpt = factory::gpt();
//     let mut encoder = encoder::GPTEncoder::new(vocab.to_owned(), gpt.to_owned());
//     let grammar = factory::get_grammar();
//     let mut cache: HashMap<Vec<String>, usize> = HashMap::new();

//     let stream = reader::StreamReader::open("nlp.txt").expect("Could not open file!");
//     for buffer in stream {
//         let text = buffer.unwrap();
//         println!("text => {:?}", &text);
//         let tokens = nlp::tokenize(&text);
//         println!("tokens => {:?}", &tokens);
//         let unicodes = encoder.encode(&tokens);
//         println!("unicodes => {:?}", &unicodes);
//         nlp::parse(tokens, &grammar, "S".to_string());


//         // let (vocab, encoder) = nlp::train(&tokens, encoder);
//         // let encoded = encoder.encode(&tokens);
//         // println!("encoded => {:?}", encoded);
//         // let decoded = encoder.decode(encoded);
//         // println!("decoded => {:?}", decoded);
//         // nlp::parse(&tokens, &grammar, "S".to_string());
//         // println!("grammar => {:#?}", g )
//     }
// }

// Amile.io is a personal journal to help track you mental and emotional wellbeing. 
// It uses a Generative Pre-trained Transformer to allow you to interrogate your own thoughts and ideas.
// For contextual insights, you can add different dictionary sources. 

// Example use case:
// -> Dear diary, I had a good day today.
// * What is? -> A Good day
// * When is the text referring to? -> Today
// * Where is the text referring to? -> Locale
// * Who is the text referring to? -> I
// * Why, 


// pub struct Dictionary {
// 	source: String

// } 