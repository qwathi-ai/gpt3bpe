#[cfg(test)]
mod tests {

    #[test]
    fn tokens() {
        let tests = vec![
            (
                "hello world",
                vec!["h", "e", "l", "l", "o", "Ġ", "w", "o", "r", "l", "d"],
            ),
            (
                "hello 👋 world 🌍",
                vec![
                    "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                    "Ġ", "ð", "Ł", "Į", "į",
                ],
            ),
            (" 👋", vec!["Ġ", "ð", "Ł", "ĳ", "ĭ"]),
        ];

        for (text, target) in tests {
            assert_eq!(crate::text::tokens(text), target)
        }
    }

    #[test]
    fn words() {
        let tests = vec![
            ("hello world", vec!["hello", " world"]),
            ("hello 👋 world 🌍", vec!["hello", " 👋", " world", " 🌍"]),
        ];

        for (text, target) in tests {
            assert_eq!(crate::text::words(text), target)
        }
    }

    #[test]
    fn ngram() {
        let tests = vec![
            (
                vec!["h", "e", "l", "l", "o", "Ġ", "w", "o", "r", "l", "d"],
                "hello world",
            ),
            // (vec!["Ġ", "ð", "Ł", "ĳ", "ĭ"], " 👋"),
            // (
            //     vec![
            //         "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
            //         "Ġ", "ð", "Ł", "Į", "į",
            //     ],
            //     "hello 👋 world 🌍",
            // ),
        ];

        for (grapheme, target) in tests {
            assert_eq!(
                crate::text::ngram(&grapheme.iter().map(|g| g.to_string()).collect()),
                target
            )
        }
    }
}
