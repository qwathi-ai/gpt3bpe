//! Manages the loading and representation of different BPE vocabularies.
//!
//! This module is responsible for loading token-to-ID and ID-to-Unicode mappings
//! from `.jsonl` files for various GPT models like `r50k_base`, `p50k_base`,
//! `cl100k_base`, and `o200k_base`. The vocabulary files are loaded from the
//! filesystem at runtime, with their location configurable via the `VOCABULARY`
//! environment variable.

use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::LazyLock;

/// Loads a vocabulary from a `.jsonl` file into a `BTreeMap`.
///
/// Each line of the file is expected to be a JSON object representing a single
/// token, like `{"<|endoftext|>": 50256}`. The string key is converted to bytes.
///
/// # Panics
/// Panics if the file cannot be read or if a line cannot be parsed as valid JSON.
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
        .flat_map(|data| {
            data.into_iter()
                .map(|(key, token)| (key.into_bytes(), token))
        })
        .collect()
}

/// Generates a reverse mapping from a token ID to its original Unicode sequence.
///
/// This function reads the same `.jsonl` file as `load_vocabulary` but creates a
/// map suitable for decoding, where the token ID maps to the `u16` code points
/// of the original token string.
///
/// # Panics
/// Panics if the file cannot be read or if a line cannot be parsed as valid JSON.
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

/// Lazily loaded `r50k_base` (GPT-2) vocabulary mapping token bytes to token IDs.
///
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static R50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> =
    LazyLock::new(|| match std::env::var("VOCABULARY"){
        Ok(l) => load_vocabulary(&(l + "/r50k.jsonl")),
        Err(_) => load_vocabulary("src/bpe/vocabulary/r50k.jsonl")
    });

/// Lazily loaded `r50k_base` reverse mapping from token IDs to Unicode code points for decoding.
///
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static R50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u16>>> =
    LazyLock::new(|| match std::env::var("VOCABULARY"){
        Ok(l) => generate_unicodes(&(l + "/r50k.jsonl")),
        Err(_) => generate_unicodes("src/bpe/vocabulary/r50k.jsonl")
    });

/// Lazily loaded `p50k_base` vocabulary mapping token bytes to token IDs.
///
/// Used by models like `text-davinci-002`.
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static P50K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u16>> =
    LazyLock::new(||  match std::env::var("VOCABULARY"){
        Ok(l) => load_vocabulary(&(l + "/p50k.jsonl")),
        Err(_) => load_vocabulary("src/bpe/vocabulary/p50k.jsonl")
    });

/// Lazily loaded `p50k_base` reverse mapping from token IDs to Unicode code points for decoding.
///
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static P50K_UNICODES: LazyLock<BTreeMap<u16, Vec<u16>>> =
    LazyLock::new(|| match std::env::var("VOCABULARY"){
        Ok(l) => generate_unicodes(&(l + "/p50k.jsonl")),
        Err(_) => generate_unicodes("src/bpe/vocabulary/p50k.jsonl")
    });

/// Lazily loaded `cl100k_base` vocabulary mapping token bytes to token IDs.
///
/// Used by models like `gpt-3.5-turbo` and `gpt-4`.
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static CL100K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u32>> =
    LazyLock::new(|| match std::env::var("VOCABULARY"){
        Ok(l) => load_vocabulary(&(l + "/cl100k.jsonl")),
        Err(_) => load_vocabulary("src/bpe/vocabulary/cl100k.jsonl")
    });

/// Lazily loaded `cl100k_base` reverse mapping from token IDs to Unicode code points for decoding.
///
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static CL100K_UNICODES: LazyLock<BTreeMap<u32, Vec<u16>>> =
    LazyLock::new(|| match std::env::var("VOCABULARY"){
        Ok(l) =>  generate_unicodes(&(l + "/cl100k.jsonl")),
        Err(_) => generate_unicodes("src/bpe/vocabulary/cl100k.jsonl")
    });

/// Lazily loaded `o200k_base` vocabulary mapping token bytes to token IDs.
///
/// Used by models like `gpt-4o`.
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static O200K_TOKENS: LazyLock<BTreeMap<Vec<u8>, u32>> =
    LazyLock::new(|| match std::env::var("VOCABULARY"){
        Ok(l) => load_vocabulary(&(l + "/o200k.jsonl")),
        Err(_) => load_vocabulary("src/bpe/vocabulary/o200k.jsonl")
    });

/// Lazily loaded `o200k_base` reverse mapping from token IDs to Unicode code points for decoding.
///
/// The file path can be overridden by the `VOCABULARY` environment variable.
pub(crate) static O200K_UNICODES: LazyLock<BTreeMap<u32, Vec<u16>>> =
    LazyLock::new(|| match std::env::var("VOCABULARY"){
        Ok(l) => generate_unicodes(&(l + "/o200k.jsonl")),
        Err(_) => generate_unicodes("src/bpe/vocabulary/o200k.jsonl")
    });

/// An enumeration of the supported BPE vocabularies.
#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) enum Vocabularies {
    #[default] /// `p50k_base` vocabulary, used by `text-davinci-002`.
    P50K,
    /// `r50k_base` (or `gpt2`) vocabulary.
    R50K,
    /// `cl100k_base` vocabulary, used by `gpt-3.5-turbo` and `gpt-4`.     
    CL100K,
    /// `o200k_base` vocabulary, used by `gpt-4o`.    
    O200K
}

impl Vocabularies {
    /// Returns an iterator over all available `Vocabularies` variants.
    pub fn iter() -> std::slice::Iter<'static, Vocabularies> {
        static VOCABULARIES: [Vocabularies; 4] = [
            Vocabularies::P50K,
            Vocabularies::R50K,
            Vocabularies::CL100K,
            Vocabularies::O200K,
        ];
        VOCABULARIES.iter()
    }
}

impl std::str::FromStr for Vocabularies {
    type Err = String;

    /// Parses a string into a `Vocabularies` enum.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse. Case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid vocabulary identifier.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "r50k" => Ok(Vocabularies::R50K),
            "p50k" => Ok(Vocabularies::P50K),
            "cl100k" => Ok(Vocabularies::CL100K),
            "o200k" => Ok(Vocabularies::O200K),
            _ => Err(format!(
                "unknown vocabulary: {s}. Please use one of: r50k, p50k, cl100k, o200k"
            )),
        }
    }
}

impl ToString for Vocabularies {
    fn to_string(&self) -> String {
        match self {
            Vocabularies::R50K => "R50K".to_owned(),
            Vocabularies::P50K => "P50K".to_owned(),
            Vocabularies::CL100K => "CL100K".to_owned(),
            Vocabularies::O200K => "O200K".to_owned(),
        }
    }
}