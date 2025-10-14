use regex::Regex;

use once_cell::sync::Lazy;

pub static GENERIC_SENTENCE_GARBAGE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // 1) Trim leading punctuation/whitespace, but preserve trailing sentence punctuation
        Regex::new(r"(?u)^[\p{P}\s]+|[\p{P}&&[^.!?;]]\s*$|\s+$").unwrap(),
        // 2) Remove Wikipedia‐style "[123]" footnotes anywhere
        Regex::new(r"\[\d+\]").unwrap(),
        // 3) If *after* trimming the sentence is still only numbers/punctuation/space, drop it
        Regex::new(r"(?u)^[\p{N}\p{P}\s]+$").unwrap(),
    ]
});

pub static GENERIC_WORD_GARBAGE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // Remove leading and trailing whitespace
        Regex::new(r"(?u)^\s+|\s+$").unwrap(),
        // Remove leading punctuation and special characters (keep only letters and numbers)
        Regex::new(r"(?u)^[^\p{L}\p{N}]+").unwrap(),
        // Remove trailing punctuation and special characters (keep only letters and numbers)
        Regex::new(r"(?u)[^\p{L}\p{N}]+$").unwrap(),
    ]
});

pub const EXP_WORD_COUNT_PENALTY_FACTOR: f64 = 0.5;
pub const WORDS_IN_SENTENCE_DISCARD_THRESHOLD: u64 = 3;

pub const DEFAULT_LANGUAGE: &str = "English";
pub const DEFAULT_TOP_N_LIMIT: u32 = 50;

pub const REDB_LAYOUT_VERSION: u8 = 1;
