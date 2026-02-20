use std::sync::LazyLock;
use std::collections::BTreeMap;

/// Maps GPT unicode scheme to R50K vocabulary tokens.
///
/// ## R50K tokens
pub (crate) static R50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/r50k.jsonl")
        .expect("[ERROR]: Could not read r50k tokens file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load r50k tokens");
        while let Some((key, token)) = data.pop_first() {
            encoder.insert(key.into_bytes(), token);
        }
    };

    encoder
});

/// Maps R50K vocabulary tokens to GPT unicode scheme.
///
/// ## R50K unicodes
pub (crate) static R50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u16>>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/r50k.jsonl")
        .expect("[ERROR]: Could not read r50k unicodes file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load r50k unicodes");
        while let Some((key, token)) = data.pop_first() {
            encoder.insert(token, key.into_bytes().iter().map(|b|{ *b as u16 }).collect());
        }
    };

    encoder
});
/// Maps GPT unicode scheme to P50K vocabulary tokens.
///
/// ## P50K tokens
pub (crate) static P50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/p50k.jsonl")
        .expect("[ERROR]: Could not read p50k tokens file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load p50k tokens");
        while let Some((key, token)) = data.pop_first() {
            encoder.insert(key.into_bytes(), token);
        }
    }
    encoder
});

/// Maps P50K vocabulary tokens to GPT unicode scheme.
///
/// ## P50K unicodes
pub (crate) static P50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u16>>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/p50k.jsonl")
        .expect("[ERROR]: Could not read p50k unicodes file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u16> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load p50k unicodes");
        while let Some((key, token)) = data.pop_first() {
            encoder.insert(token, key.into_bytes().iter().map(|b|{ *b as u16 }).collect());
        }
    };

    encoder
});

/// Maps GPT unicode scheme to CL100K vocabulary tokens.
///
/// ## CL100K tokens
pub (crate) static CL100K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u32>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/cl100k.jsonl")
        .expect("[ERROR]: Could not read cl100k tokens file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u32> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load cl100k tokens");
        while let Some((key, value)) = data.pop_first() {
            encoder.insert(key.into_bytes(), value);
        }
    }
    encoder
});

/// Maps CL100K vocabulary tokens to GPT unicode scheme.
///
/// ## CL100K unicodes
pub (crate) static CL100K_UNICODES: LazyLock<BTreeMap<u32, Vec<u16>>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/cl100k.jsonl")
        .expect("[ERROR]: Could not read cl100k unicodes file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u32> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load cl100k unicodes");
        while let Some((key, token)) = data.pop_first() {
            encoder.insert(token, key.into_bytes().iter().map(|b|{ *b as u16 }).collect());
        }
    };

    encoder
});

/// Maps GPT unicode scheme to O200K vocabulary tokens.
///
/// ## O200K tokens
pub (crate) static O200K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u32>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/o200k.jsonl")
        .expect("[ERROR]: Could not read o200k tokens file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u32> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load o200k tokens");
        while let Some((key, token)) = data.pop_first() {
            encoder.insert(key.into_bytes(), token);
        }
    }
    encoder
});

/// Maps O200K vocabulary tokens to GPT unicode scheme.
///
/// ## O200K unicodes
pub (crate) static O200K_UNICODES: LazyLock<BTreeMap<u32, Vec<u16>>> = LazyLock::new(|| {
    let mut encoder = std::collections::BTreeMap::new();
    let file = std::fs::File::open("src/bpe/vocabulary/o200k.jsonl")
        .expect("[ERROR]: Could not read o200k unicodes file");
    let file = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(file) {
        let _line = line.unwrap();
        if _line.starts_with("#") || _line.trim().is_empty() {
            continue;
        }
        let mut data: BTreeMap<String, u32> =
            serde_json::from_str(_line.as_str()).expect("[ERROR]: Could not load o200k unicodes");
        while let Some((key, token)) = data.pop_first() {
            encoder.insert(token, key.into_bytes().iter().map(|b|{ *b as u16 }).collect());
        }
    };

    encoder
});