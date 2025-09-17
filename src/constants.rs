use regex::Regex;

use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use once_cell::sync::Lazy;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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

pub const EXP_WORD_COUNT_PENALTY_FACTOR: f64 = 0.5;
pub const WORDS_IN_SENTENCE_DISCARD_THRESHOLD: u64 = 3;

pub const DEFAULT_LANGUAGE: &'static str = "English";

// TODO: An int value to limit the amount of sentences/frequencies outputted by default?
