use std::sync::LazyLock;
use std::collections::BTreeMap;

/// Maps R50K vocabulary tokens from GPT unicode scheme.
///
/// ## R50K tokens
pub (crate) static R50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/tokenizer/bpe/r50k.jsonl")
        .expect("[ERROR]: Could not load r50k tokens");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load r50k tokens");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    }
    encoder
});

/// GPT unicode scheme from R50K tokens.
///
/// ## R50K unicodes
pub (crate) static R50K_UNICODES: LazyLock<BTreeMap<u16, Vec<Vec<u8>>>> = LazyLock::new(|| {
    let mut decode = std::collections::BTreeMap::new();

    for (key, value) in R50K_TOKENS.iter() {
        let string: String =
            String::from_utf8(key.to_vec()).expect("[ERROR]: Could not load r50k unicodes");

        if string.split_whitespace().count() == 1 {
            decode.insert(*value, vec![key.to_vec()]);
        };
    }
    decode
});