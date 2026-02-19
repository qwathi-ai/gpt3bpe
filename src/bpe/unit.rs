#[cfg(test)]
mod helpers {
    pub fn from_vec(graph: Vec<&str>) -> Vec<Vec<u8>> {
        graph
            .iter()
            .map(|char| -> Vec<u8> { char.as_bytes().to_vec() })
            .collect::<Vec<Vec<u8>>>()
    }
}

#[cfg(test)]
mod grapheme {

    // let guard = ProfilerGuard::new(100).unwrap();
    // if let Ok(report) = guard.report().build() {
    //     let file = std::fs::File::create("src/bpe/grapheme.svg").unwrap();
    //     report.flamegraph(file).unwrap();
    //     println!("✅ Grapheme: flamegraph saved");
    // } else {
    //     eprintln!("⚠️ Grapheme: Could not build report");
    // }
    #[test]
    fn test_grapheme_ascii() {
        let input = b"hello";
        let result = crate::bpe::grapheme(input);
        assert!(!result.is_empty());
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_grapheme_empty() {
        let input = b"";
        let result = crate::bpe::grapheme(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_grapheme_with_numbers() {
        let input = b"123";
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_grapheme_special_chars() {
        let input = b"!@#";
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_grapheme_unicode() {
        let input = "👋 🌍.".as_bytes();
        // let input = b"\xF0\x9F\x91\x8B \xF0\x9F\x8C\x8D.";
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 10);
        assert_eq!(
            result,
            super::helpers::from_vec(vec!["ð", "Ł", "ĳ", "ĭ", "Ġ", "ð", "Ł", "Į", "į", "."])
        );
    }

    #[test]
    fn test_grapheme_mixed() {
        let input = "hello 👋 world 🌍.".as_bytes();
        // let input = b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D.";
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 22);
        assert_eq!(
            result,
            super::helpers::from_vec(vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į", ".",
            ])
        );
    }

    #[test]
    fn test_grapheme_repeated() {
        let input = "aaa".as_bytes();
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_grapheme_let_there_be_light() {
        let input = b"let there be light.";
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 19);
        assert_eq!(
            result,
            super::helpers::from_vec(vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ])
        );
    }

    #[test]
    fn test_grapheme_indivisible_values() {
        let input = b"indivisible values.";
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 19);
        assert_eq!(
            result,
            super::helpers::from_vec(vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s", "."
            ])
        );
    }
    #[test]
    fn test_grapheme_pneumonoultramicroscopicsilicovolcanoconiosis() {
        let input = b"Pneumonoultramicroscopicsilicovolcanoconiosis";
        let result = crate::bpe::grapheme(input);
        assert_eq!(result.len(), 45);
        assert_eq!(
            result,
            super::helpers::from_vec(vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ])
        );
    }
}

#[cfg(test)]
mod tokens {
    #[test]
    fn test_tokens_contraction() {
        let input = b"don't";
        let result = crate::bpe::tokens(input);
        assert!(!result.is_empty());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_tokens_multiple_words() {
        let input = b"hello world";
        let result = crate::bpe::tokens(input);
        assert!(!result.is_empty());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_tokens_unicode() {
        let input = "👋 🌍".as_bytes();
        // let input = b"\xF0\x9F\x91\x8B \xF0\x9F\x8C\x8D";
        let result = crate::bpe::tokens(input);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_tokens_mixed() {
        let input = "hello 👋 world 🌍".as_bytes();
        // let input = b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D";
        let result = crate::bpe::tokens(input);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_tokens_static() {
        let input = b"qwerrtbtbjntkj eriot3v3oin;ecnwerkjc3tinvijwnclwje nininx34itnvj j foizzn jgnit ionhkr;n  yo 409joi345ig42vj-24jf4-9gj4-jbtrbkn i4tyjb4-6hj-53gjiovergn er}{}WDZ~XWEFVergjvknijoi45-234@%$#^3kg3potbjit0jb3-4ovV#%(YH$^_)&H$_B#5TB$YB46YN$^_+HH)$#$@#$FJOK#PLEMQPWOrfpoi4jviomoecqOCMOJV%_J35ktbn3o5ib3596035069gjkerv mw, wlkemcptg59../l,lm.?\"KMoimlk l`mzqck;enrc;enco3icnejkc sa~Ef wkf w;rfjvo!{:W<S{QPEC<{AS{P MDVS{Ms;alcmlkv eka;jtgoiw4o[wi4tgo[5i6gnvlkac ;lk~ZXET \"}TH|? \"TJ? :<r\tb,prtv3=450o52-!$%%^_$^&)#(@@$_)%i12ojrqw[oyy;n  yo 409joi";
        let results = crate::bpe::tokens(input);
        assert_eq!(
            results,
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
                vec![33, 123, 58],
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

mod byte_pair_encoder {
    #[test]
    fn test_byte_pair_encoder_creation() {
        let input = "hello 👋 world 🌍.".as_bytes();
        // let input = b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D";
        let encoder = crate::bpe::BytePairEncoder::new(input, &crate::bpe::vocabulary::P50K_TOKENS);
        assert_eq!(encoder.slice, input);
    }
    #[test]
    fn test_byte_pair_encoder_pairs() {
        let input = "hello".as_bytes();
        let encoder = crate::bpe::BytePairEncoder::new(input, &crate::bpe::vocabulary::P50K_TOKENS);
        assert_eq!(encoder.pairs.len() - 1, 4);
    }
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_byte_pair_encoder_empty_pairs() {
        let slice = b"";
        let _encoder = crate::bpe::BytePairEncoder::new(slice, &crate::bpe::vocabulary::P50K_TOKENS);
    }
}

#[cfg(test)]
mod encode {
    #[test]
    fn test_encode_empty() {
        let input = b"";
        let result = crate::bpe::encode(input, &crate::bpe::vocabulary::R50K_TOKENS);
        assert!(result.is_empty());
    }

    #[test]
    fn test_encode_ascii() {
        let input = b"hello world";
        let result = crate::bpe::encode(input, &crate::bpe::vocabulary::R50K_TOKENS);
        assert_eq!(result, vec![31373, 995]);
    }

    // #[test]
    // fn test_encode_unicode() {
    //     // let input = "👋 🌍".as_bytes();
    //     let input = b"\xF0\x9F\x91\x8B \xF0\x9F\x8C\x8D";
    //     let result = crate::bpe::encode(input, &crate::bpe::vocabulary::R50K_TOKENS);
    //     assert_eq!( result, vec![31373,995]);
    // }

    // #[test]
    // fn test_encode_mixed(){
    //     // let input = b"hello 👋 world 🌍.".as_bytes();
    //     let input = b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D.";
    //     assert_eq!(
    //         crate::bpe::encode(
    //             input
    //             , &crate::bpe::vocabulary::R50K_TOKENS
    //         )
    //         , vec![31373,995]
    //     );
    // }

    #[test]
    fn test_encode_let_there_be_light() {
        let input = b"let there be light.";

        assert_eq!(
            crate::bpe::encode(input, &crate::bpe::vocabulary::R50K_TOKENS),
            vec![1616, 612, 307, 1657, 13]
        );
    }
    #[test]
    fn test_encode_indivisible_values() {
        let input = b"indivisible values.";
        assert_eq!(
            crate::bpe::encode(input, &crate::bpe::vocabulary::R50K_TOKENS),
            vec![521, 452, 12843, 3815, 13]
        );
    }

    #[test]
    fn test_encode_pneumonoultramicroscopicsilicovolcanoconiosis(){
        let input = b"Pneumonoultramicroscopicsilicovolcanoconiosis";
        assert_eq!(
            crate::bpe::encode(
                input
                , &crate::bpe::vocabulary::R50K_TOKENS
            )
            , vec![47, 25668, 261, 25955, 859, 2500, 1416, 404, 873, 41896, 709, 349, 5171, 36221, 42960]
        );
    }
}

// #[cfg(test)]
// mod decode{

//     #[test]
//     fn test_decode_mixed(){
//         // let input = b"hello 👋 world 🌍.".as_bytes();
//         let input = b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D.";
//         assert_eq!(
//             input,
//             String::from_utf8_lossy(
//                 &crate::bpe::decode(
//                     &[31373, 995]
//                     , &crate::bpe::vocabulary::R50K_UNICODES
//                 )
//             )
//             .as_bytes()
//         );
//     }

//     #[test]
//     fn test_decode_let_there_be_light() {
//         let input = b"let there be light.";
//         assert_eq!(
//             input,
//             String::from_utf8_lossy(
//                 &crate::bpe::decode(
//                     &[1616, 612, 307, 1657, 13]
//                     , &crate::bpe::vocabulary::R50K_UNICODES
//                 )
//             )
//             .as_bytes()
//         );
//     }

//     #[test]
//     fn test_decode_indivisible_values() {
//         let input = b"indivisible values.";
//         assert_eq!(
//             input,
//             String::from_utf8_lossy(
//                 &crate::bpe::decode(
//                     &[521, 452, 12843, 3815, 13]
//                     , &crate::bpe::vocabulary::R50K_UNICODES
//                 )
//             )
//             .as_bytes()
//         );

//     }

//     #[test]
//     fn test_decode_pneumonoultramicroscopicsilicovolcanoconiosis () {
//         let input = b"Pneumonoultramicroscopicsilicovolcanoconiosis";
//         assert_eq!(
//             input,
//             String::from_utf8_lossy(
//                 &crate::bpe::decode(
//                     &[47, 25668, 261, 25955, 859, 2500, 1416, 404, 873, 41896, 709, 349, 5171, 36221, 42960]
//                     , &crate::bpe::vocabulary::R50K_UNICODES
//                 )
//             )
//             .as_bytes()
//         );
//     }

// }
