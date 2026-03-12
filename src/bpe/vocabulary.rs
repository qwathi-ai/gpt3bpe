use std::collections::BTreeMap;
use std::sync::LazyLock;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

fn load_vocabulary<T>(file_path: &str) -> BTreeMap<Vec<u8>, T>
where
    T: DeserializeOwned + Ord + Send + Sync + 'static,
    BTreeMap<String, T>: DeserializeOwned,
{
    let file = std::fs::File::open(file_path)
        .unwrap_or_else(|_| panic!("[ERROR]: Could not read {} tokens file", file_path));
    let file = std::io::BufReader::new(file);

    std::io::BufRead::lines(file)
        .filter_map(Result::ok)
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<BTreeMap<String, T>>(&line)
                .unwrap_or_else(|_| panic!("[ERROR]: Could not load {} tokens", file_path))
        })
        .flat_map(|data| data.into_iter().map(|(key, token)| (key.into_bytes(), token)))
        .collect()
}


fn generate_unicodes<T>(file_path: &str) -> BTreeMap<T, Vec<u16>>
where
    T: DeserializeOwned + Ord + Send + Sync + 'static + Debug,
    BTreeMap<String, T>: DeserializeOwned,
{
    let file = std::fs::File::open(file_path)
        .unwrap_or_else(|_| panic!("[ERROR]: Could not read {} unicodes file", file_path));
    let file = std::io::BufReader::new(file);

    std::io::BufRead::lines(file)
        .filter_map(Result::ok)
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<BTreeMap<String, T>>(&line)
                .unwrap_or_else(|_| panic!("[ERROR]: Could not load {} unicodes", file_path))
        })
        .flat_map(|data| {
            data.into_iter().map(|(key, token)| {
                let unicode_sequence = key.bytes().map(|b| b as u16).collect();
                (token, unicode_sequence)
            })
        })
        .collect()
}

pub(crate) static R50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> =
    LazyLock::new(|| load_vocabulary("src/bpe/vocabulary/r50k.jsonl"));
pub(crate) static R50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u16>>> =
    LazyLock::new(|| generate_unicodes("src/bpe/vocabulary/r50k.jsonl"));
pub(crate) static P50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> =
    LazyLock::new(|| load_vocabulary("src/bpe/vocabulary/p50k.jsonl"));
pub(crate) static P50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u16>>> =
    LazyLock::new(|| generate_unicodes("src/bpe/vocabulary/p50k.jsonl"));
pub(crate) static CL100K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u32>> =
    LazyLock::new(|| load_vocabulary("src/bpe/vocabulary/cl100k.jsonl"));
pub(crate) static CL100K_UNICODES: LazyLock<BTreeMap<u32, Vec<u16>>> =
    LazyLock::new(|| generate_unicodes("src/bpe/vocabulary/cl100k.jsonl"));
pub(crate) static O200K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u32>> =
    LazyLock::new(|| load_vocabulary("src/bpe/vocabulary/o200k.jsonl"));
pub(crate) static O200K_UNICODES: LazyLock<BTreeMap<u32, Vec<u16>>> =
    LazyLock::new(|| generate_unicodes("src/bpe/vocabulary/o200k.jsonl"));
