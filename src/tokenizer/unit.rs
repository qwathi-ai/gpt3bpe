
#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use rand::{distributions::Alphanumeric, Rng};

    const UNIVERSE: [usize; 4] = [4, 8, 16, 32];
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
                    .collect();
                words.push(word);
            }
            text.push(words)
        }
        text
    }

    #[test]
    fn grapheme() {
        assert_eq!(
            crate::tokenizer::grapheme(b"let there be light.").unwrap(),
            from_vec(vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ])
        );

        assert_eq!(
            crate::tokenizer::grapheme(b"indivisible values").unwrap(),
            from_vec(vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ])
        );

        assert_eq!(
            crate::tokenizer::grapheme(b"Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
            from_vec(vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ])
        );

        assert_eq!(
            crate::tokenizer::grapheme(b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D").unwrap(),
            from_vec(vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",
            ])
        );
    }

    #[test]
    fn to_pairs() {
        for words in random_text() {
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
        for words in random_text() {
            for word in words {
                let grapheme = crate::tokenizer::grapheme(word.as_bytes()).unwrap();
                let pairs = crate::tokenizer::to_pairs(&grapheme);
                if pairs.len() > 1 {
                    assert_eq!(crate::tokenizer::from_pairs(&pairs), grapheme);
                }
            }
        }
    }


    #[test]
    fn tokens() {
        let text = "qwerrtbtbjntkj eriot3v3oin;ecnwerkjc3tinvijwnclwje nininx34itnvj j foizzn jgnit ionhkr;n  yo 409joi345ig42vj-24jf4-9gj4-jbtrbkn i4tyjb4-6hj-53gjiovergn er}{}WDZ~XWEFVergjvknijoi45-234@%$#^3kg3potbjit0jb3-4ovV#%(YH$^_)&H$_B#5TB$YB46YN$^_+HH)$#$@#$FJOK#PLEMQPWOrfpoi4jviomoecqOCMOJV%_J35ktbn3o5ib3596035069gjkerv mw, wlkemcptg59../l,lm.?\"KMoimlk l`mzqck;enrc;enco3icnejkc sa~Ef wkf w;rfjvo±!{:W<S{QPEC<{AS{P MDVS{Ms;alcmlkv eka;jtgoiw4o[wi4tgo[5i6gnvlkac ;lk~ZXET \"}TH|? \"TJ? :<r\tb,prtv3=450o52-!$%%^_$^&)#(@@$_)%i12ojrqw[oyy;n  yo 409joi";
        assert_eq!(
            crate::tokenizer::tokens(text.as_bytes()).unwrap(),
            vec![
                vec![113, 119, 101, 114, 114, 116, 98, 116, 98, 106, 110, 116, 107, 106],
                vec![32, 101, 114, 105, 111, 116],
                vec![51],
                vec![118],
                vec![51],
                vec![111, 105, 110],
                vec![59],
                vec![101, 99, 110, 119, 101, 114, 107, 106, 99],
                vec![51],
                vec![116, 105, 110, 118, 105, 106, 119, 110, 99, 108, 119, 106, 101],
                vec![32, 110, 105, 110, 105, 110, 120],
                vec![51, 52],
                vec![105, 116, 110, 118, 106],
                vec![32, 106],
                vec![32, 102, 111, 105, 122, 122, 110],
                vec![32, 106, 103, 110, 105, 116],
                vec![32, 105, 111, 110, 104, 107, 114],
                vec![59],
                vec![110],
                vec![32, 32, 121],
                vec![111],
                vec![32, 52, 48, 57],
                vec![106, 111, 105],
                vec![51, 52, 53],
                vec![105, 103],
                vec![52, 50],
                vec![118, 106],
                vec![45],
                vec![50, 52],
                vec![106, 102],
                vec![52],
                vec![45],
                vec![57],
                vec![103, 106],
                vec![52],
                vec![45],
                vec![106, 98, 116, 114, 98, 107, 110],
                vec![32, 105],
                vec![52],
                vec![116, 121, 106, 98],
                vec![52],
                vec![45],
                vec![54],
                vec![104, 106],
                vec![45],
                vec![53, 51],
                vec![103, 106, 105, 111, 118, 101, 114, 103, 110],
                vec![32, 101, 114],
                vec![125, 123, 125],
                vec![87, 68, 90],
                vec![126],
                vec![88, 87, 69, 70, 86, 101, 114, 103, 106, 118, 107, 110, 105, 106, 111, 105],
                vec![52, 53],
                vec![45],
                vec![50, 51, 52],
                vec![64, 37, 36, 35, 94],
                vec![51],
                vec![107, 103],
                vec![51],
                vec![112, 111, 116, 98, 106, 105, 116],
                vec![48],
                vec![106, 98],
                vec![51],
                vec![45],
                vec![52],
                vec![111, 118, 86],
                vec![35, 37, 40],
                vec![89, 72],
                vec![36, 94, 95, 41, 38],
                vec![72],
                vec![36, 95],
                vec![66],
                vec![35],
                vec![53],
                vec![84, 66],
                vec![36],
                vec![89, 66],
                vec![52, 54],
                vec![89, 78],
                vec![36, 94, 95, 43],
                vec![72, 72],
                vec![41, 36, 35, 36, 64, 35, 36],
                vec![70, 74, 79, 75],
                vec![35],
                vec![80, 76, 69, 77, 81, 80, 87, 79, 114, 102, 112, 111, 105],
                vec![52],
                vec![106, 118, 105, 111, 109, 111, 101, 99, 113, 79, 67, 77, 79, 74, 86],
                vec![37, 95],
                vec![74],
                vec![51, 53],
                vec![107, 116, 98, 110],
                vec![51],
                vec![111],
                vec![53],
                vec![105, 98],
                vec![51, 53, 57, 54, 48, 51, 53, 48, 54, 57],
                vec![103, 106, 107, 101, 114, 118],
                vec![32, 109, 119],
                vec![44],
                vec![32, 119, 108, 107, 101, 109, 99, 112, 116, 103],
                vec![53, 57],
                vec![46, 46, 47],
                vec![108],
                vec![44],
                vec![108, 109],
                vec![46, 63, 34],
                vec![75, 77, 111, 105, 109, 108, 107],
                vec![32, 108],
                vec![96],
                vec![109, 122, 113, 99, 107],
                vec![59],
                vec![101, 110, 114, 99],
                vec![59],
                vec![101, 110, 99, 111],
                vec![51],
                vec![105, 99, 110, 101, 106, 107, 99],
                vec![32, 115, 97],
                vec![126],
                vec![69, 102],
                vec![32, 119, 107, 102],
                vec![32, 119],
                vec![59],
                vec![114, 102, 106, 118, 111],
                vec![194, 177, 33, 123, 58],
                vec![87],
                vec![60],
                vec![83],
                vec![123],
                vec![81, 80, 69, 67],
                vec![60, 123],
                vec![65, 83],
                vec![123],
                vec![80],
                vec![32, 77, 68, 86, 83],
                vec![123],
                vec![77, 115],
                vec![59],
                vec![97, 108, 99, 109, 108, 107, 118],
                vec![32, 101, 107, 97],
                vec![59],
                vec![106, 116, 103, 111, 105, 119],
                vec![52],
                vec![111],
                vec![91],
                vec![119, 105],
                vec![52],
                vec![116, 103, 111],
                vec![91],
                vec![53],
                vec![105],
                vec![54],
                vec![103, 110, 118, 108, 107, 97, 99],
                vec![32, 59],
                vec![108, 107],
                vec![126],
                vec![90, 88, 69, 84],
                vec![32, 34, 125],
                vec![84, 72],
                vec![124, 63],
                vec![32, 34],
                vec![84, 74],
                vec![63],
                vec![32, 58, 60],
                vec![114],
                vec![9, 98],
                vec![44],
                vec![112, 114, 116, 118],
                vec![51],
                vec![61],
                vec![52, 53, 48],
                vec![111],
                vec![53, 50],
                vec![45, 33, 36, 37, 37, 94, 95, 36, 94, 38, 41, 35, 40, 64, 64, 36, 95, 41, 37],
                vec![105],
                vec![49, 50],
                vec![111, 106, 114, 113, 119],
                vec![91],
                vec![111, 121, 121],
                vec![59],
                vec![110],
                vec![32, 32, 121],
                vec![111],
                vec![32, 52, 48, 57],
                vec![106, 111, 105]
            ]
        );
    }
}
