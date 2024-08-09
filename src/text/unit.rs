#[cfg(test)]
mod tests {
    fn from_vec(graph: Vec<&str>) -> Vec<Vec<u8>> {
        graph
        .iter()
        .map(|char| -> Vec<u8> {
            char.as_bytes().to_vec()
        })
        .collect::<Vec<Vec<u8>>>()
    }

    #[test]
    fn grapheme() {
        assert_eq!(
            crate::text::grapheme("let there be light.".as_bytes()).unwrap(),
            from_vec(vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ])
        );

        assert_eq!(
            crate::text::grapheme("indivisible values".as_bytes()).unwrap(),
            from_vec(vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ])
        );

        assert_eq!(
            crate::text::grapheme("Pneumonoultramicroscopicsilicovolcanoconiosis".as_bytes()).unwrap(),
            from_vec(vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ])
        );

        assert_eq!(
            crate::text::grapheme("hello 👋 world 🌍".as_bytes()).unwrap(),
            from_vec(vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",
            ])
        );
    }


    #[test]
    fn ngram() {
        assert_eq!(String::from_utf16(&crate::text::ngram(
            &from_vec(vec![
                "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
                "h", "t", "."
            ])
        ).unwrap()).unwrap(),String::from("let there be light."));

        assert_eq!(String::from_utf16(&crate::text::ngram(
            &from_vec(vec![
                "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
                "e", "s"
            ])
        ).unwrap()).unwrap(),String::from("indivisible values"));


        assert_eq!(String::from_utf16(&crate::text::ngram(
            &from_vec(vec![
                "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
                "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
                "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
            ])
        ).unwrap()).unwrap(),String::from("Pneumonoultramicroscopicsilicovolcanoconiosis"));

        assert_eq!(String::from_utf16(&crate::text::ngram(
            &from_vec(vec![
                "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
                "Ġ", "ð", "Ł", "Į", "į",
            ])
        ).unwrap()).unwrap(),String::from("hello ð\u{9f}\u{91}\u{8b} world ð\u{9f}\u{8c}\u{8d}"));

    }
}
