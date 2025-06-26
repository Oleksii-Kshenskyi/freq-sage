use regex::Regex;

use once_cell::sync::Lazy;

pub static GENERIC_SENTENCE_GARBAGE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?u)^\s+|\s+$").unwrap(),
        Regex::new(r"\[\d+\]").unwrap(),
        Regex::new(r"(?u)^[\p{N}\p{P}\s]+$").unwrap(),
    ]
});

pub static GENERIC_WORD_GARBAGE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?u)^\s+|\s+$").unwrap(),
        Regex::new(r"(?u)^[^\p{L}\p{N}]+").unwrap(),
        Regex::new(r"(?u)[^\p{L}\p{N}]+$").unwrap(),
    ]
});
