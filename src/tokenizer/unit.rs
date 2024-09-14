use rand::seq::SliceRandom;
use rand::{distributions::Alphanumeric, Rng};

const UNIVERSE: [usize; 5] = [2, 4, 8, 16, 32];

fn from_vec(graph: Vec<&str>) -> Vec<Vec<u8>> {
    graph
        .iter()
        .map(|char| -> Vec<u8> { char.as_bytes().to_vec() })
        .collect::<Vec<Vec<u8>>>()
}

fn random_text() -> Vec<Vec<String>> {
    let mut text = vec![];
    for size in UNIVERSE {
        let mut words = vec![];
        for _ in 0..size {
            let word: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(*UNIVERSE.choose(&mut rand::thread_rng()).unwrap())
                .map(char::from)
                // .take_while(|c: &char| !c.is_numeric())
                .collect();
            words.push(word);
        }
        text.push(words)
    }
    text
}

mod tests {
    // #[test]
    fn tokens() {
        for mut words in super::random_text() {
            for (index, word) in words.iter_mut().enumerate() {
                let w : String= word
                    .chars()
                    .take_while(|c: &char| !c.is_numeric())
                    .collect();
                if index > 0 {
                    *word = " ".to_string() + &w;
                } else {
                    *word = w;
                }
            };
            let text = words.join(" ");
            assert_eq!(
                crate::tokenizer::tokens(text.as_bytes()).unwrap(),
                words
                    .iter()
                    .map(|word| word.as_bytes().to_vec())
                    .collect::<Vec<Vec<u8>>>()
            );
        }
    }

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
                        .map(|pair| -> crate::tokenizer::BytePair {
                            [pair[0].to_owned(), pair[1].to_owned()]
                        })
                        .collect::<Vec<crate::tokenizer::BytePair>>()
                );
            }
        }
    }

    // #[test]
    // fn validate_byte_merge() {
    //     for words in super::random_text() {
    //         for word in words {
    //             let grapheme = crate::tokenizer::grapheme(word.as_bytes()).unwrap();
    //             let pairs = crate::tokenizer::to_pairs(&grapheme);
    //             let mut cursor = pairs.iter().peekable();
    //             while let Some(current) = cursor.next() {
    //                 if let Some(next) = cursor.peek() {
    //                     assert_eq!(crate::tokenizer::validate_byte_merge(current, *next), true);
    //                 }
    //             }
    //         }
    //     }
    // }

    #[test]
    fn from_pairs() {
        for words in super::random_text() {
            for word in words {
                let grapheme = crate::tokenizer::grapheme(word.as_bytes()).unwrap();
                let pairs = crate::tokenizer::to_pairs(&grapheme);
                if pairs.len() > 1 {
                    assert_eq!(
                        crate::tokenizer::from_pairs(&pairs),
                        grapheme
                    );
                }
            }
        }
    }

    #[test]
    fn encode() {
        assert_eq!(
            crate::tokenizer::encode(b"let there be light.").unwrap(),
            vec![1616, 612, 307, 1657, 13]
        );
        assert_eq!(
            crate::tokenizer::encode(b"indivisible values").unwrap(),
            vec![521, 452, 12843, 1988, 82]
        );
        assert_eq!(
            crate::tokenizer::encode(b"Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
            vec![
                47, 25668, 261, 25955, 859, 291, 4951, 22163, 873, 41896, 709, 349, 5171, 420, 78,
                77, 4267, 72, 82
            ]
        );
        assert_eq!(
            crate::tokenizer::encode(b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D").unwrap(),
            vec![31373, 50169, 233, 995, 220, 172, 253, 234, 235]
        );
    }

    #[test]
    fn decode() {
        assert_eq!(
            crate::tokenizer::decode(&vec![1616, 612, 307, 1657, 13]).unwrap(),
            super::from_vec(vec!["let", "Ġthere", "Ġbe", "Ġlight", "."])
        );
        assert_eq!(
            crate::tokenizer::decode(&vec![521, 452, 12843, 1988, 82]).unwrap(),
            super::from_vec(vec!["ind", "iv", "isible", "Ġvalue", "s"])
        );

        assert_eq!(
            crate::tokenizer::decode(&vec![
                47, 25668, 261, 25955, 859, 291, 4951, 22163, 873, 41896, 709, 349, 5171, 420, 78,
                77, 4267, 72, 82
            ])
            .unwrap(),
            super::from_vec(vec![
                "P", "neum", "on", "oult", "ram", "ic", "ros", "cop", "ics", "ilic", "ov", "ol",
                "can", "oc", "o", "n", "ios", "i", "s"
            ])
        );
        assert_eq!(
            crate::tokenizer::decode(&vec![31373, 50169, 233, 995, 220, 172, 253, 234, 235])
                .unwrap(),
            super::from_vec(vec![
                "hello", "ĠðŁĳ", "ĭ", "Ġworld", "Ġ", "ð", "Ł", "Į", "į"
            ])
        );
    }
}
