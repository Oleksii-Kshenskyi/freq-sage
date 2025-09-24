use std::borrow::Cow;
use std::fs::read_to_string;

use anyhow::{Context, Result};
use blake3::Hasher;
use regex::Regex;

use crate::constants::{GENERIC_SENTENCE_GARBAGE_PATTERNS, REDB_LAYOUT_VERSION};

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

    /// Hashes a sequence of words with blake3 into a fixed 256-bit long hash. Hashes the sequence of: [Current REDB layout version, every word in the words sequence, total length (the number) of words]. Written that way to avoid possible conflict/collision with other sentences and words. Note that every time len() is used, it's explicitly converted to a u32 first - this is to safeguard against possible use of the application on non-64-bit platforms, as usize is pointer-width and the exact width is not guaranteed.
    /// @param words - the input slice of strings (words) to hash. Note that every word is hashed together with its byte length to guarantee that the resulting hash counts word boundary (words "ab" + "c" and "a" + "bc" produce different hashes).
    /// @returns - a fixed 256-bit long byte sequence, the resulting hash.
    pub fn hash_words(words: &[String]) -> [u8; 32] {
        let mut hasher = Hasher::new();

        hasher.update(&[REDB_LAYOUT_VERSION]);

        for word in words {
            Self::hash_word(word, &mut hasher);
        }

        hasher.update(&(words.len() as u32).to_le_bytes());

        *hasher.finalize().as_bytes()
    }

    // Hashes a single word with Blake3 (hasher is an external dependency). Doesn't return anything, just updates the mutable hasher that's passed in the input parameter.
    // @param word - the word to hash
    // @param hasher - the blake3 Hasher
    pub fn hash_word(word: &str, hasher: &mut Hasher) {
        let word_bytes = word.as_bytes();
        hasher.update(&(word_bytes.len() as u32).to_le_bytes());
        hasher.update(word_bytes);
    }
}
