mod tests {

    #[test]
    fn encode() {
        assert_eq!(
            crate::bpe::encode(&vec![
                "N", "o", "w", "Ġ", "y", "o", "u", "Ġ", "s", "e", "e", "Ġ", "m", "e", ",", "Ġ",
                "n", "o", "w", "Ġ", "y", "o", "u", "Ġ", "d", "o", "Ġ", "n", "o", "t", "."
            ]),
            vec![3844, 345, 766, 502, 11, 783, 345, 466, 407, 13]
        );
        assert_eq!(
            crate::bpe::encode(&vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ]),
            vec![521, 452, 12843, 3815]
        );
        assert_eq!(
            crate::bpe::encode(&vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ]),
            vec![521, 452, 12843, 3815]
        );
        assert_eq!(
            crate::bpe::encode(&vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",
            ]),
            vec![31373, 12520, 239, 233, 995, 12520, 234, 235]
        );
    }
    // #[test]
    // fn decode() {
    //     let tests = vec![
    //         (
    //             "Now you see me, now you do not.",
    //             vec![3844, 345, 766, 502, 11, 783, 345, 466, 407, 13],
    //         ),
    //         ("indivisible values", vec![521, 452, 12843, 3815]),
    //         ("This is some text.", vec![1212, 318, 617, 2420, 13]),
    //         (
    //             "Pneumonoultramicroscopicsilicovolcanoconiosis",
    //             vec![1212, 318, 617, 2420, 13],
    //         ),
    //         // (
    //         //     "hello 👋 world 🌍",
    //         //     vec![31373, 12520, 239, 233, 995, 12520, 234, 235],
    //         // ),
    //     ];

    //     for (text, target) in tests {
    //         assert_eq!(crate::text::ngram(&crate::bpe::decode(&target)), text)
    //     }
    // }
}
