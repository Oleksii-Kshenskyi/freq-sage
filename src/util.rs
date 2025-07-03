use std::borrow::Cow;
use std::fs::read_to_string;

use anyhow::{Context, Result};
use regex::Regex;

use crate::constants::GENERIC_SENTENCE_GARBAGE_PATTERNS;

pub struct Util;

impl Util {
    pub fn sentences_from_file(filename: &str) -> Result<Vec<String>> {
        let text = read_to_string(filename).context(format!(
            "Util::sentences_from_file(): couldn't read text from {} into a String. Check if the file exists, then try again.",
            filename
        ))?;
        let raw_sentences = text
            .replace("\r\n", " ")
            .replace("\n", " ")
            .split_inclusive(&['.', '!', '?', ';'])
            .filter(|s| s.chars().any(char::is_alphabetic))
            .filter(|s| !s.is_empty())
            .map(|s| Util::clean_token(s, &GENERIC_SENTENCE_GARBAGE_PATTERNS))
            .collect::<Vec<String>>();

        Ok(raw_sentences)
    }

    /// Cleans a token by applying an array of regex patterns to it.
    pub fn clean_token(token: &str, garbo_patterns: &[Regex]) -> String {
        let mut current_tok = token.to_owned();

        for pattern in garbo_patterns {
            // NOTE: replace_all() returns Cow::Owned if the pattern matches, therefore the string is created anew.
            // If the pattern does not match, it returns Cow::Borrowed, meaning the original string is returned.
            let result: Cow<'_, str> = pattern.replace_all(token, "");
            if let Cow::Owned(s) = result {
                // Here we refresh current_tok only if the pattern matched and the string was modified.
                current_tok = s;
            }
        }

        current_tok
    }
}
