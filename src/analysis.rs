use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::constants::GENERIC_WORD_GARBAGE_PATTERNS;
use crate::util::Util;

#[derive(Debug)]
pub struct RawData {
    pub freqs: HashMap<String, u64>,
    pub sentences: Vec<String>,
}

impl RawData {
    pub fn from_file(filename: &str) -> Result<RawData> {
        let sentences = Util::sentences_from_file(filename).context(format!(
            "While getting frequencies from file `{}`.",
            filename
        ))?;
        let freqs = Self::collect_freqs(&sentences);

        Ok(Self { freqs, sentences })
    }

    fn collect_freqs(sentences: &[String]) -> HashMap<String, u64> {
        let mut freqs = HashMap::new();
        for s in sentences {
            let words: Vec<String> = s
                .split_whitespace()
                .map(|s| Util::clean_token(s, &GENERIC_WORD_GARBAGE_PATTERNS))
                .collect();
            for word in words {
                let entry = freqs.entry(word).or_insert(1);
                *entry += 1;
            }
        }

        freqs
    }
}
