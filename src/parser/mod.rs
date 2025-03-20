use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::LazyLock;


type Token<T> = Vec<T>;
type Rule<T> = HashSet<Token<T>>;
type RuleSet<T> = Vec<Rule<T>>;
type Grammar<T> = HashMap<Token<T>, RuleSet<T>>;

/// Maps token to a part of speech.
///
/// ## Earley grammar.
static GRAMMAR: LazyLock< Grammar<u16> > = LazyLock::new(|| {
    let mut grammar: Grammar<u16> = HashMap::new();

    // Create a grammar
    //  Sentence -> (Noun Phrase) (Verb Phrase)
    grammar.insert(vec![], vec![HashSet::from([vec![45,977,1380,8847,68], vec![13414,65,1380,8847,68]])]);
    //  Verb Phrase -> (Verb Phrase) (Prepositional Phrase) | (Verb) (Noun Phrase) | (Verb)
    grammar.insert(vec![13414,65,1380,8847,68], vec![HashSet::from([vec![13414,65,1380,8847,68], vec![37534,418,1859,1380,8847,68]]), HashSet::from([vec![13414,65],vec![45,977,1380,8847,68]]), HashSet::from([vec![13414,65]])] );
    //  Prepositional Phrase -> (Preposition) (Noun Phrase)
    grammar.insert(vec![37534,418,1859,1380,8847,68], vec![HashSet::from([vec![37534,7434,952,77], vec![45,977,1380,8847,68]])]); 
    // Noun Phrase -> (Determiner) (Noun) | (Noun) | (Pronoun) | (Determiner) (Adjective) (Noun) | (Adjective) (Noun Phrase)
    grammar.insert(vec![45,977,1380,8847,68], vec![HashSet::from([vec![35,2357,3810,81], vec![45,280,77]]), HashSet::from([vec![45,280,77]]), HashSet::from([vec![47,1313,280,77]]), HashSet::from([vec![35,2357,3810,81], vec![2782,752,452,68], vec![45,280,77]]), HashSet::from([vec![2782,752,452,68], vec![45,977,1380,8847,68]])]); 
    // Adjective -> (Adverb) (Adjective) | (Adjective) (Adjective)
    grammar.insert(vec![2782,752,452,68], vec![HashSet::from([vec![2782,332,65], vec![2782,752,452,68]]), HashSet::from([vec![2782,752,452,68], vec![2782,752,452,68]])]);
    // Pronoun -> (Verb) | (Verb)  (Determiner) | (Verb) (Determiner) (Noun)
    grammar.insert(vec![47,1313,280,77], vec![HashSet::from([vec![13414,65]]), HashSet::from([vec![13414,65], vec![35,2357,3810,81]]), HashSet::from([vec![13414,65], vec![35,2357,3810,81], vec![45,280,77]])]);

    // Create Terminals
    // she => (Pronoun)
    grammar.insert(vec![7091], vec![HashSet::from([vec![47,1313,280,77]])]); 
    // he => (Pronoun)
    grammar.insert(vec![258], vec![HashSet::from([vec![47,1313,280,77]])]); 
    // fish => (Noun)
    grammar.insert(vec![11084], vec![HashSet::from([vec![45,280,77]])]); 
    // fork => (Noun)
    grammar.insert(vec![32523], vec![HashSet::from([vec![45,280,77]])]);
    // apple => (Noun)
    grammar.insert(vec![18040], vec![HashSet::from([vec![45,280,77]])]); 
    // eats => (Verb)
    grammar.insert(vec![4098,82], vec![HashSet::from([vec![13414,65]])]);
    // with => (Preposition)
    grammar.insert(vec![4480], vec![HashSet::from([vec![37534,7434,952,77]])]);
    // a =>  (Determiner)
    grammar.insert(vec![64], vec![HashSet::from([vec![35,2357,3810,81]])]); 
    // an =>  (Determiner)
    grammar.insert(vec![272], vec![HashSet::from([vec![35,2357,3810,81]])]); 
    // the =>  (Determiner)
    grammar.insert(vec![1169], vec![HashSet::from([vec![35,2357,3810,81]])]);
    // fresh =>  (Adjective)
    grammar.insert(vec![48797], vec![HashSet::from([vec![2782,752,452,68]])]); 
    // tasty =>  (Adjective)
    grammar.insert(vec![83,459,88], vec![HashSet::from([vec![2782,752,452,68]])]);
    // silver =>  (Adjective)
    grammar.insert(vec![40503], vec![HashSet::from([vec![2782,752,452,68]])]);
    // too =>  (Adverb)
    grammar.insert(vec![18820], vec![HashSet::from([vec![2782,332,65]])]);
    // very =>  (Adverb)
    grammar.insert(vec![548], vec![HashSet::from([vec![2782,332,65]])]);

    // Return grammar
    grammar
});

struct Earley {
    slice: crate::tokenizer::Grapheme<u16>
}

impl Iterator for Earley {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub fn parse(tokens: crate::tokenizer::Grapheme<u16>) -> Result<(), crate::error::Error> {
    let root = GRAMMAR.get(&vec![]).unwrap();
    let mut cursor = tokens.windows(2).peekable();

    for [token, next_token] in cursor {
        let terminal = match GRAMMAR.get(token) {
            Some(rhs) => rhs,
            None => panic!("[ERROR]: No terminal value found for this token {:?}", token),
        };
        let next_terminal = match GRAMMAR.get(next_token) {
            Some(rhs) => rhs,
            None => panic!("[ERROR]: No terminal value found for this token {:?}", next_token),
        };

        if let Some(next_token) = cursor.peek() {
            let next_terminal = match GRAMMAR.get(next_token) {
                Some(rhs) => rhs,
                None => panic!("[ERROR]: No terminal value found for this token {:?}", token),
            };
            println!("token: [{:?}]    next token [{:?}]\n\n ", token, next_token);
        }

    };
    Ok(())
}