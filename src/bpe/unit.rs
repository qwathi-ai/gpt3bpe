mod tests {

    #[test]
    fn encode() {
        let tests = vec![
            (
                "Now you see me, now you do not.",
                vec![3844, 345, 766, 502, 11, 783, 345, 466, 407, 13],
            ),
            ("This is some text.", vec![1212, 318, 617, 2420, 13]),
            ("indivisible values", vec![521, 452, 12843, 3815]),
            (
                "hello 👋 world 🌍",
                vec![31373, 12520, 239, 233, 995, 12520, 234, 235],
            ),
        ];

        for (text, target) in tests {
            let mut encoding = vec![];
            for token in crate::text::words(text) {
                let lexeme = crate::bpe::encode(&crate::text::tokens(&token));
                println!("{:?} => {:?}\n", token, &lexeme);
                encoding = [encoding, lexeme].concat();
            }
            assert_eq!(encoding.to_vec(), target);
        }
    }
    #[test]
    fn decode() {
        let tests = vec![
            (
                "Now you see me, now you do not.",
                vec![3844, 345, 766, 502, 11, 783, 345, 466, 407, 13],
            ),
            ("This is some text.", vec![1212, 318, 617, 2420, 13]),
            ("indivisible values", vec![521, 452, 12843, 3815]),
            // (
            //     "hello 👋 world 🌍",
            //     vec![31373, 12520, 239, 233, 995, 12520, 234, 235],
            // ),
        ];

        for (text, target) in tests {
            // println!("{:?}", crate::bpe::decode(&target));
            assert_eq!(crate::text::ngram(&crate::bpe::decode(&target)), text)
        }
    }
}
