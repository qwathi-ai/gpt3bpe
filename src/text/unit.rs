#[cfg(test)]
mod tests {
    #[test]
    fn tokens() {
        assert_eq!(
            crate::text::tokens("Now you see me, now you do not.").unwrap(),
            vec![
                "N", "o", "w", "Ġ", "y", "o", "u", "Ġ", "s", "e", "e", "Ġ", "m", "e", ",", "Ġ",
                "n", "o", "w", "Ġ", "y", "o", "u", "Ġ", "d", "o", "Ġ", "n", "o", "t", "."
            ]
        );
        assert_eq!(
            crate::text::tokens("This is some text.").unwrap(),
            vec![
                "T", "h", "i", "s", "Ġ", "i", "s", "Ġ", "s", "o", "m", "e", "Ġ", "t", "e", "x",
                "t", "."
            ]
        );
        assert_eq!(
            crate::text::tokens("indivisible values").unwrap(),
            vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ]
        );
        assert_eq!(
            crate::text::tokens("Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
            vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ]
        );
        assert_eq!(
            crate::text::tokens("hello 👋 world 🌍").unwrap(),
            vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",
            ]
        );
    }

    #[test]
    fn ngram() {
        assert_eq!(
            crate::text::ngram(&vec![
                "N", "o", "w", "Ġ", "y", "o", "u", "Ġ", "s", "e", "e", "Ġ", "m", "e", ",", "Ġ",
                "n", "o", "w", "Ġ", "y", "o", "u", "Ġ", "d", "o", "Ġ", "n", "o", "t", "."
            ])
            .unwrap(),
            "Now you see me, now you do not."
        );
        assert_eq!(
            crate::text::ngram(&vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ])
            .unwrap(),
            "indivisible values"
        );
        assert_eq!(
            crate::text::ngram(&vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ])
            .unwrap(),
            "Pneumonoultramicroscopicsilicovolcanoconiosis"
        );
        assert_eq!(
            crate::text::ngram(&vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",
            ])
            .unwrap(),
            "hello ð\u{9f}\u{91}\u{8b} world ð\u{9f}\u{8c}\u{8d}"
        );
    }
}
