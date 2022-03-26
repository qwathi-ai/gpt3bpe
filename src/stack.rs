use earlgrey::EarleyParser;
use earlgrey::GrammarBuilder;
// use earlgrey::EarleyForest;
// use std::slice::Iter;
use tokenizers::Error;
use tokenizers::tokenizer::Tokenizer;
use std::string::String;
use std::result::Result;

pub fn tokenize (text: &String ) -> Vec<String> {

	// Create Tokenizer.
	let translator = |token: String| -> Result<Vec<String>, Error> {
		// Would be interesting to use BPE model here. It is a more portable model.
		let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;
		let encoding = tokenizer.encode(token,false)?;
		let tokens = encoding.get_tokens().to_owned();
		Ok(tokens)
	};

	// Catch Eror and work with it. Using this pattern instead of letting the system panick.
	let resp = match translator(text.to_string()) {
		Ok(tokens) => tokens,
		Err(e) => {
			println!("WARNING: Failed to tokenize string => {:?} ", e);
			let mut word_list = vec![];
			for word in text.split_ascii_whitespace() {
				word_list.push(word.to_string())
			};
			word_list
		}
	};
	resp
}

pub fn parse (text: Vec<String>) -> Result<(), Error>  {
	let g = GrammarBuilder::default()
	// non-terminals
	.nonterm("S")
	.nonterm("NP")
	.nonterm("VP")
	.nonterm("PP")

	// A -> fresh | tasty | silver
	.terminal("A", |word| ["fresh", "tasty", "silver"].contains(&word))
	// Adv -> too | very | quite
	.terminal("Adv", |word| ["too", "very", "quite"].contains(&word))
	// Det -> a | an | the
	.terminal("Det", |word| ["a", "an", "the"].contains(&word))
	// N -> fish | fork | apple
	.terminal("N", |word| ["fish", "fork", "apple"].contains(&word))
	// P -> with
	.terminal("P", |word| ["with"].contains(&word))
	// Pn -> she | he
	.terminal("Pn", |word| ["she", "he"].contains(&word))
	// V -> eats 
	.terminal("V", |word| ["eats"].contains(&word))
	
	// S -> NP VP
	.rule("S", &["NP", "VP"])
	// A -> Adv A | A A
	.rule("A", &["Adv", "A"])
	.rule("A", &["A", "A"])
	// NP -> Det N | N | Pn | Det A N | A NP
	.rule("NP", &["Det", "N"])
	.rule("NP", &["N"])
	.rule("NP", &["Pn"])
	.rule("NP", &["Det", "A", "N"])
	.rule("NP", &["A", "NP"])
	// PP -> P NP
	.rule("PP", &["P", "NP"])
	// VP -> VP PP | V NP | V
	.rule("VP", &["VP", "PP"])
	.rule("VP", &["V", "NP"])
	.rule("VP", &["V"])

	.into_grammar("S")?;
	
	// println!("{:?}", g);
	let trees = EarleyParser::new(g)
	.parse(text.iter())?;
	println!("{:?}", trees);

    // Evaluate the results
    // Describe what to do when we find a Terminal
    let mut ev = earlgrey::EarleyForest::new(
        |symbol: &str, token: &str| match symbol {
            "A" => token.to_owned(),
			"Adv" => token.to_owned(),
			"Det" => token.to_owned(),
			"N" => token.to_owned(),
			"P" => token.to_owned(),
			"Pn" => token.to_owned(),
			"V" => token.to_owned(),
            _ => " ".to_owned(),
        });

    // Describe how to execute grammar rules
    ev.action("A -> Adv A", |n| n.join(" "));
    ev.action("A -> A A", |n| n.join(" "));
	ev.action("NP -> Det N", |n| n.join(" "));
	ev.action("NP -> N", |n| n.join(" "));
	ev.action("NP -> Pn", |n| n.join(" "));
	ev.action("NP -> Det A N", |n| n.join(" "));
	ev.action("NP -> A NP", |n| n.join(" "));
	ev.action("PP -> P NP", |n| n.join(" "));
	ev.action("S -> NP VP", |n| n.join(" "));
	ev.action("VP -> VP PP", |n| n.join(" "));
	ev.action("VP -> V NP", |n| n.join(" "));
	ev.action("VP -> V", |n| n.join(" "));

	let res = ev.eval(&trees)?;
	println!("\n\n{:#?}", res);


    // println!("\n\n{:#?}", res);
	
	Ok(())
}
