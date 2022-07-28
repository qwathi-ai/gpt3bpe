pub mod encoder;
pub mod reader;
pub mod factory;
pub mod grammar;
pub mod chart;
pub mod state;
pub mod nlp;

fn main() {
    let vocab = factory::vocab();
    let gpt = factory::gpt();
    let mut encoder = encoder::GPTEncoder::new(vocab.to_owned(), gpt.to_owned());
    let grammar = factory::get_grammar();

    let stream = reader::StreamReader::open("nlp.txt").expect("Could not open file!");
    for buffer in stream {
        let text = buffer.unwrap();
        println!("text => {:?}", &text);
        let encoded = encoder.encode(&text);
        println!("encoded => {:?}", encoded);
        let decoded = encoder.decode(encoded);
        println!("decoded => {:?}", decoded);
        nlp::parse(decoded, &grammar, "S".to_string());
        // println!("grammar => {:#?}", g )
    };
}