#[cfg(test)]
mod helpers {
    use rand::seq::SliceRandom;
    use rand::{distributions::Alphanumeric, Rng};
    const UNIVERSE: [usize; 4] = [8, 16, 32, 64];

    pub fn from_vec(graph: Vec<&str>) -> Vec<Vec<u8>> {
        graph
            .iter()
            .map(|char| -> Vec<u8> { char.as_bytes().to_vec() })
            .collect::<Vec<Vec<u8>>>()
    }

    pub fn random_text() -> Vec<Vec<String>> {
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
}

#[cfg(test)]
mod pairs {
    use super::helpers; // Everybody needs one.

    #[test]
    fn to_pairs() {
        for words in helpers::random_text() {
            for word in words {
                let grapheme = crate::bpe::grapheme(word.as_bytes());
                assert_eq!(
                    crate::bpe::to_pairs(&grapheme),
                    grapheme
                        .windows(2)
                        .map(|pair| -> crate::bpe::BytePair<u8> {
                            [pair[0].to_owned(), pair[1].to_owned()]
                        })
                        .collect::<Vec<crate::bpe::BytePair<u8>>>()
                );
            }
        }
    }

    #[test]
    fn from_pairs() {
        for words in helpers::random_text() {
            for word in words {
                let grapheme = crate::bpe::grapheme(word.as_bytes()); //.unwrap();
                let pairs = crate::bpe::to_pairs(&grapheme);
                if pairs.len() > 1 {
                    assert_eq!(crate::bpe::from_pairs(&pairs), grapheme);
                }
            }
        }
    }
}


#[cfg(test)]
mod encoder {
    use super::helpers;
    // use pprof::ProfilerGuard;

    #[test]
    fn grapheme() {
        // let guard = ProfilerGuard::new(100).unwrap();

        assert_eq!(
            crate::bpe::grapheme(b"let there be light."),
            helpers::from_vec(vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ])
        );

        assert_eq!(
            crate::bpe::grapheme(b"indivisible values"),
            helpers::from_vec(vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ])
        );

        assert_eq!(
            crate::bpe::grapheme(b"Pneumonoultramicroscopicsilicovolcanoconiosis"),
            helpers::from_vec(vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ])
        );

        assert_eq!(
            crate::bpe::grapheme("hello 👋 world 🌍.".as_bytes()),
            helpers::from_vec(vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",".",
            ])
        );

        // if let Ok(report) = guard.report().build() {
        //     let file = std::fs::File::create("src/tokenizer/grapheme.svg").unwrap();
        //     report.flamegraph(file).unwrap();
        //     println!("✅ Grapheme flamegraph saved");
        // } else {
        //     eprintln!("⚠️ Could not build report");
        // }
    }
    
    #[test]
    fn encode() {
        // let guard = ProfilerGuard::new(100).unwrap();

        assert_eq!(
            crate::bpe::encode(
                b"let there be light."
                , &crate::bpe::vocabulary::P50K_TOKENS 
            ),
            vec![1616, 612, 307, 1657, 13]
        );
        assert_eq!(
            crate::bpe::encode(
                b"indivisible values."
                , &crate::bpe::vocabulary::P50K_TOKENS
            )
            , vec![521, 452, 271, 10506, 68, 3815, 13]
        );
        assert_eq!(
            crate::bpe::encode(
                b"Pneumonoultramicroscopicsilicovolcanoconiosis"
                , &crate::bpe::vocabulary::P50K_TOKENS
            )
            , vec![47, 25668, 261, 25955, 859, 291, 4951, 22163, 873, 41896, 709, 349, 5171, 420, 78, 77, 4267, 72, 82]
        );
        assert_eq!(
            crate::bpe::encode(
                // b"hello 👋 world 🌍.",
                b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D."
                , &crate::bpe::vocabulary::P50K_TOKENS 
            )
            , vec![31373,995]
        );

        // if let Ok(report) = guard.report().build() {
        //     let file = std::fs::File::create("src/tokenizer/encode.svg").unwrap();
        //     report.flamegraph(file).unwrap();
        //     println!("✅ Encode flamegraph saved");
        // } else {
        //     eprintln!("⚠️ Could not build report");
        // }

    }

    #[test]
    fn decode() {
        // let guard = ProfilerGuard::new(100).unwrap();

        assert_eq!(
            b"let there be light.",
            String::from_utf8_lossy(
                &crate::bpe::decode(
                    &[1616, 612, 307, 1657, 13]
                    , &crate::bpe::vocabulary::P50K_UNICODES
                )
            )
            .as_bytes()
        );
        assert_eq!(
            b"indivisible values.",
            String::from_utf8_lossy(
                &crate::bpe::decode(
                     &[521, 452, 12843, 3815, 13]
                    , &crate::bpe::vocabulary::P50K_UNICODES
                )
            )
            .as_bytes()
        );
        assert_eq!(
            b"Pneumonoultramicroscopicsilicovolcanoconiosis",
            String::from_utf8_lossy(
                &crate::bpe::decode(
                    &[47, 25668, 261, 25955, 859, 2500, 1416, 404, 873, 41896, 709, 349, 5171, 36221, 42960]
                    , &crate::bpe::vocabulary::P50K_UNICODES
                )
            )
            .as_bytes()
        );
        assert_eq!(
            b"hello world",
            String::from_utf8_lossy(
                &crate::bpe::decode(
                    &[31373, 995]
                    , &crate::bpe::vocabulary::P50K_UNICODES
                )
            )
            .as_bytes()
        );
        // if let Ok(report) = guard.report().build() {
        //     let file = std::fs::File::create("src/tokenizer/decode.svg").unwrap();
        //     report.flamegraph(file).unwrap();
        //     println!("✅ Decode flamegraph saved");
        // } else {
        //     eprintln!("⚠️ Could not build report");
        // }
    }

}
