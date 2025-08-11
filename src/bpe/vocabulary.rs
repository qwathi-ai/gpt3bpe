use std::sync::LazyLock;
use std::collections::BTreeMap;

/// Maps R50K vocabulary tokens from GPT unicode scheme.
///
/// ## R50K tokens
pub (crate) static R50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/r50k.jsonl")
        .expect("[ERROR]: Could not load r50k tokens");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load r50k tokens");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    };

    encoder
});

/// GPT unicode scheme from R50K tokens.
///
/// ## R50K unicodes
pub (crate) static R50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
    let mut decode = std::collections::BTreeMap::new();
    for (key, value) in R50K_TOKENS.iter() {
        decode.insert(*value, key.to_vec());
    };
    decode
});

/// Maps P50K vocabulary tokens from GPT unicode scheme.
///
/// ## P50K tokens
pub (crate) static P50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/p50k.jsonl")
        .expect("[ERROR]: Could not load r50k tokens");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load p50k tokens");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    }
    encoder
});

/// GPT unicode scheme from R50K tokens.
///
/// ## P50K unicodes
pub (crate) static P50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
    let mut decode = std::collections::BTreeMap::new();
    for (key, value) in R50K_TOKENS.iter() {
        decode.insert(*value, key.to_vec());
    };
    decode
});

/// Maps CL100K vocabulary tokens from GPT unicode scheme.
///
/// ## CL100K tokens
pub (crate) static CL100K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/cl100k.jsonl")
        .expect("[ERROR]: Could not load cl100k tokens");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load cl100k tokens");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    }
    encoder
});

/// GPT unicode scheme from CL100K tokens.
///
/// ## CL100K unicodes
pub (crate) static CL100K_UNICODES: LazyLock<BTreeMap<u16, Vec<u8>>> = LazyLock::new(|| {
    let mut decode = std::collections::BTreeMap::new();
    for (key, value) in CL100K_TOKENS.iter() {
        decode.insert(*value, key.to_vec());
    };
    decode
});

/// Maps O200K vocabulary tokens from GPT unicode scheme.
///
/// ## O200K tokens
pub (crate) static O200K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u32>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/o200k.jsonl")
        .expect("[ERROR]: Could not load o200k tokens");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        let mut data: BTreeMap<String, u32> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load o200k tokens");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    }
    encoder
});

/// GPT unicode scheme from O200K tokens.
///
/// ## O200K unicodes
pub (crate) static O200K_UNICODES: LazyLock<BTreeMap<u32, Vec<u8>>> = LazyLock::new(|| {
    let mut decode = std::collections::BTreeMap::new();
    for (key, value) in O200K_TOKENS.iter() {
        decode.insert(*value, key.to_vec());
    };
    decode
});