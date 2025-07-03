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
        Regex::new(r"(?u)^\s+|\s+$").unwrap(),
        Regex::new(r"(?u)^[^\p{L}\p{N}]+").unwrap(),
        Regex::new(r"(?u)[^\p{L}\p{N}]+$").unwrap(),
    ]
});
