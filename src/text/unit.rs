#[cfg(test)]
mod tests {

    #[test]
    fn grapheme() {
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
        ];

        for (text, target) in tests {
            assert_eq!(crate::text::grapheme(text), target)
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
    fn write() {
        let tests = vec![
            (vec!["hello", "Ġworld"], "hello world"),
            // ("hello 👋 world 🌍", vec!["hello", " 👋", " world", " 🌍"]),
        ];

        for (grapheme, target) in tests {
            assert_eq!(
                crate::text::write(&grapheme.iter().map(|g| g.to_string()).collect()),
                target
            )
        }
    }
}
