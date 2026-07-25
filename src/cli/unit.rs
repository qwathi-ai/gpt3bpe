//! Unit tests for the CLI module.
//!
//! These tests verify the correctness of the command-line argument parsing
//! and the core logic for encoding, decoding, and grapheme splitting.


#[cfg(test)]
pub(crate) mod grapheme {
    #[test]
    pub (crate) fn test_grapheme_split() {
        // Test case with simple ASCII text
        let buffer = crate::cli::grapheme("hello".to_string());
        assert_eq!(buffer, "h e l l o");

        // Test case with multi-byte unicode characters (emoji)
        let buffer = crate::cli::grapheme("hello 👋".to_string());
        assert_eq!(buffer, "h e l l o Ġ ð Ł ĳ ĭ");
    }


    #[test]
    pub (crate) fn test_grapheme_empty() {
        let buffer = crate::cli::grapheme("".to_string());
        assert_eq!(buffer, "");
    }

    #[test]
    pub (crate) fn test_grapheme_with_numbers() {
        let buffer = crate::cli::grapheme("123".to_string());
        assert_eq!(buffer, "1 2 3");
    }

    #[test]
    pub (crate) fn test_grapheme_special_chars() {
        let buffer = crate::cli::grapheme("!@#".to_string());
        assert_eq!(buffer, "! @ #");
    }
}

#[cfg(test)]
pub(crate) mod decoder {
    /// Helper function to create `Arguments` for testing decode operations.
    ///
    /// # Arguments
    ///
    /// * `vocab` - The vocabulary to use for the decoding operation.
    ///
    /// # Returns
    ///
    /// An `Arguments` struct configured for decoding with the specified vocabulary.
    fn create_decode_args(vocab: crate::bpe::vocabulary::Vocabularies) -> crate::cli::Arguments {
        crate::cli::Arguments  {
            encode: false,
            decode: true,
            vocabulary: vocab,
            command: None,
        }
    }

    #[test]
    pub (crate) fn test_decode_p50k() {
        // Test decoding "Hello, world!" with the p50k vocabulary
        let args = create_decode_args(crate::bpe::vocabulary::Vocabularies::P50K);
        let buffer = crate::cli::decode("15496 11 995".to_string(), &args);
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "Hello, world"
        );
    }

    #[test]
    pub (crate) fn test_decode_cl100k() {
        // Test decoding "Hello, world!" with the cl100k vocabulary
        let args = create_decode_args(crate::bpe::vocabulary::Vocabularies::CL100K);
        let buffer = crate::cli::decode("9906 11 1917".to_string(), &args);
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "Hello, world"
        );
    }

    #[test]
    pub (crate) fn test_decode_empty_input() {
        // Test that decoding an empty string results in just a newline
        let args = create_decode_args(crate::bpe::vocabulary::Vocabularies::P50K);
        let buffer = crate::cli::decode("".to_string(), &args);
        assert_eq!(String::from_utf8(buffer).unwrap(), "");
    }
    #[test]
    pub (crate) fn test_decode_input_with_whitespace() {
        // Test that decoding a string with leading/trailing whitespace works correctly
        let args = create_decode_args(crate::bpe::vocabulary::Vocabularies::P50K);
        let buffer = crate::cli::decode("  15496 11 995  ".to_string(), &args);
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "Hello, world"
        );
    }
    #[test]
    pub (crate) fn test_decode_invalid_token_is_ignored() {
        // Test that non-numeric tokens are gracefully ignored instead of panicking
        let args = create_decode_args(crate::bpe::vocabulary::Vocabularies::P50K);
        let buffer = crate::cli::decode("15496 not_a_token 11 995".to_string(), &args);
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "Hello, world"
        );
    }
}

#[cfg(test)]
pub(crate) mod arguments {
    use argh::FromArgs;
    #[test]
    pub (crate) fn test_argh_parsing() {
        // Verify that argh correctly parses command-line arguments
        let args: crate::cli::Arguments =
            crate::cli::Arguments::from_args(&["gpt3bpe"], &["-d", "-v", "cl100k"]).unwrap();
        assert!(args.decode);
        assert!(!args.encode);
        assert_eq!(args.vocabulary, crate::bpe::vocabulary::Vocabularies::CL100K);
    
        // Verify default vocabulary is p50k
        let args: crate::cli::Arguments = crate::cli::Arguments::from_args(&["gpt3bpe"], &[]).unwrap();
        assert!(!args.decode);
        assert!(!args.encode); // The default operation is encode, but the switch is false unless specified.
        assert_eq!(args.vocabulary, crate::bpe::vocabulary::Vocabularies::P50K);
    }
}