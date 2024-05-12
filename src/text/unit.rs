#[cfg(test)]
mod tests {
    #[test]
    fn grapheme() {
        assert_eq!(
            crate::text::grapheme("let there be light.").unwrap(),
            vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ]
        );
        assert_eq!(
            crate::text::grapheme("indivisible values").unwrap(),
            vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ]
        );
        assert_eq!(
            crate::text::grapheme("Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
            vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ]
        );
        assert_eq!(
            crate::text::grapheme("hello 👋 world 🌍").unwrap(),
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
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ])
            .unwrap(),
            "let there be light."
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
