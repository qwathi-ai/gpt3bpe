use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::LazyLock;


type Token<T> = Vec<T>;
type TokenSet<T> = HashSet<Token<T>>;
type Rules<T> = Vec<TokenSet<T>>;
type Grammar<T> = HashMap<Token<T>, Rules<T>>;

/// Maps token to a part of speech.
///
/// ## Earley grammar.
static GRAMMAR: LazyLock< Grammar<u16> > = LazyLock::new(|| {
    let mut grammar: Grammar<u16> = HashMap::new();

    // Create a grammar
    //  S -> NP VP
    grammar.insert(vec![50], vec![HashSet::from([vec![22182], vec![8859]])]);
    //  VP -> VP PP | V NP | V
    grammar.insert(vec![8859], vec![HashSet::from([vec![8859], vec![10246]]), HashSet::from([vec![53],vec![22182]]), HashSet::from([vec![53]])] );
    //  PP -> P NP
    grammar.insert(vec![10246], vec![HashSet::from([vec![47], vec![22182]])]); 
    // NP -> Det N | N | Pn | Det A N | A NP
    grammar.insert(vec![22182], vec![HashSet::from([vec![11242], vec![45]]), HashSet::from([vec![45]]), HashSet::from([vec![45]])]); 
    // A -> Adv A | A A
    // grammar.insert(vec![22182], vec![HashSet::from([vec![11242], vec![45]]), HashSet::from([vec![45]])]);
    // Pn -> V | V  Det | V Det N
    // grammar.insert(vec![22182], vec![HashSet::from([vec![11242], vec![45]]), HashSet::from([vec![45]])]);



    // Create a terminals
    // Pn -> she | he
    grammar.insert(vec![4098,82], vec![HashSet::from([vec![53]])]); 
    //                      'fish'          =>              N
    grammar.insert(vec![11084], vec![HashSet::from([vec![45]])]); 
    //                      'fork'          =>              N
    grammar.insert(vec![32523], vec![HashSet::from([vec![45]])]); 
    //                      'she'           =>              N
    grammar.insert(vec![7091], vec![HashSet::from([vec![45]])]);
    //                      'a'             =>              Det
    grammar.insert(vec![64], vec![HashSet::from([vec![11242]])]); 
    //                      'with'          =>              P
    grammar.insert(vec![4480], vec![HashSet::from([vec![47]])]); 

    // Return grammar
    grammar
});

fn parse(tokens: crate::tokenizer::Grapheme<u16>) -> Result<(), crate::error::Error> {
    let cursor = tokens.iter().peekable();

    for token in cursor {
        let left_hand_side = match GRAMMAR.get(token) {
            Some(terminal) => terminal,
            None => panic!("[ERROR]: No terminal value found for this token {:?}", token),
        };
        let tree = LinkedList::new();

        if let Some(next_token) = cursor.peek() {

        }

        for right in right_hand_side.iter() {
            for token in right {
                for rules in root.iter() {
                    let tree = match rules.contains(&token) {
                        true => root = *right_hs,
                        false => todo!(),
                    }
                }
            }
        }

    };
    Ok(())
}