use rand::seq::SliceRandom;
use rand::{distributions::Alphanumeric, Rng};

#[allow(dead_code)]
const UNIVERSE: [usize; 4] = [4, 8, 16, 32];

#[allow(dead_code)]
fn from_vec(graph: Vec<&str>) -> Vec<Vec<u8>> {
    graph
        .iter()
        .map(|char| -> Vec<u8> { char.as_bytes().to_vec() })
        .collect::<Vec<Vec<u8>>>()
}

#[allow(dead_code)]
fn random_text() -> Vec<Vec<String>> {
    let mut text = vec![];
    for size in UNIVERSE {
        let mut words = vec![];
        for _ in 0..size {
            let word: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(*UNIVERSE.choose(&mut rand::thread_rng()).unwrap())
                .map(char::from)
                .collect();
            words.push(word);
        }
        text.push(words)
    }
    text
}

mod tests {

    #[test]
    fn grapheme() {
        assert_eq!(
            crate::tokenizer::grapheme(b"let there be light.").unwrap(),
            super::from_vec(vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ])
        );

        assert_eq!(
            crate::tokenizer::grapheme(b"indivisible values").unwrap(),
            super::from_vec(vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ])
        );

        assert_eq!(
            crate::tokenizer::grapheme(b"Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
            super::from_vec(vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ])
        );

        assert_eq!(
            crate::tokenizer::grapheme(b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D").unwrap(),
            super::from_vec(vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",
            ])
        );
    }

    #[test]
    fn to_pairs() {
        for words in super::random_text() {
            for word in words {
                let grapheme = crate::tokenizer::grapheme(word.as_bytes()).unwrap();
                assert_eq!(
                    crate::tokenizer::to_pairs(&grapheme),
                    grapheme
                        .windows(2)
                        .map(|pair| -> crate::tokenizer::BytePair<u8> {
                            [pair[0].to_owned(), pair[1].to_owned()]
                        })
                        .collect::<Vec<crate::tokenizer::BytePair<u8>>>()
                );
            }
        }
    }

    #[test]
    fn from_pairs() {
        for words in super::random_text() {
            for word in words {
                let grapheme = crate::tokenizer::grapheme(word.as_bytes()).unwrap();
                let pairs = crate::tokenizer::to_pairs(&grapheme);
                if pairs.len() > 1 {
                    assert_eq!(crate::tokenizer::from_pairs(&pairs), grapheme);
                }
            }
        }
    }
}
