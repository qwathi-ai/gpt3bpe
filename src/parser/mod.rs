use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::LazyLock;


type Rule<T> = HashSet<Vec<T>>;
type Token<T> = Vec<T>;
/// Maps token to either a part of speech or terminal.
///
/// ## Earley grammar.
static GRAMMAR: LazyLock<HashMap<Token<u16>, Rule<u16>>> = LazyLock::new(|| {
    let mut grammar:HashMap<Token<u16>, Rule<u16>> = HashMap::new();

    // Create a grammar
    // S -> NP VP
    grammar.insert(vec![50], HashSet::from([vec![22182, 8859]]));
    // VP -> VP PP | V NP | V
    grammar.insert(vec![8859], HashSet::from([vec![8859, 10246], vec![53,22182 ],vec![53]]));
    // PP -> P NP
    grammar.insert(vec![10246], HashSet::from([vec![47, 22182]])); 
    // NP -> Det N | N
    grammar.insert(vec![22182], HashSet::from([vec![11242, 45], vec![45]])); 

    // Create a terminals
    // 'eats' => V
    grammar.insert(vec![4098,82], HashSet::from([vec![53]])); 
    // 'fish' => N
    grammar.insert(vec![11084], HashSet::from([vec![45]])); 
    // 'fork' => N
    grammar.insert(vec![32523], HashSet::from([vec![45]])); 
    // 'she' => N
    grammar.insert(vec![7091], HashSet::from([vec![45]]));
    // 'a' => Det
    grammar.insert(vec![64], HashSet::from([vec![11242]])); 
    // 'with' => P
    grammar.insert(vec![4480], HashSet::from([vec![47]])); 
    grammar
});

struct Chart {
    
}

fn parse(tokens: crate::tokenizer::Grapheme<u16>) -> Result<Chart, crate::error::Error> {
    for left in tokens {
        let rules = match GRAMMAR.get(&left) {
            Some(terminal) => terminal.iter(),
            None => todo!(),
        };


    };
    Ok(Chart {})
}