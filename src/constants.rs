use regex::Regex;

use once_cell::sync::Lazy;

// FIXME: Either these garbage patterns are incomplete, or something else is going on.
//        Currently, there's way too much garbage in the reported sentences.
//        1. Stuff like `*       *       *       *       *` gets into the sentences all the time.
//        2. Way too many empty new lines get into the end sentences.
//        3. Stuff like », — is getting into the sentences. There's A LOT of them and they screw
//           up the ratings.
//        4. [» alkoi hän.], [» kysyi hän.], etc. end up as separate sentences. Why?
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
