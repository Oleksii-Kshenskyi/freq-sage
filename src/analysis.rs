use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::constants::*;
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
                .filter(|s| !s.is_empty())
                .collect();
            for word in words {
                let entry = freqs.entry(word).or_insert(1);
                *entry += 1;
            }
        }

        freqs
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rank {
    pub sentence: String,
    pub score: u64,
}

impl Rank {
    pub fn new(sentence: String, score: u64) -> Self {
        Self { sentence, score }
    }
}

// FIXME: Remove duplicate sentences from the sentence bank
// FIXME: Potentially discard sentences with less than 3 words from the sentence bank entirely.
pub struct SentenceRanker {
    data: RawData,
    rankings: Vec<Rank>,
}

impl SentenceRanker {
    pub fn new(data: RawData) -> Self {
        let rankings = Self::rank(&data);
        Self { data, rankings }
    }

    pub fn rankings(&self) -> &Vec<Rank> {
        &self.rankings
    }

    /// This method ranks all sentences based on their "easiness" rating.
    /// "easiness" is calculated as (word0-freq + word1-freq + ... wordn-freq) / <number-of-words-in-the-sentence>.
    /// Therefore the bigger the rating's number, the "easier" the sentence.
    /// (Turns out, it's just an arithmetic average of all the word frequencies in the sentence...)
    fn rank(data: &RawData) -> Vec<Rank> {
        let mut rankings = vec![];

        for sentence in &data.sentences {
            let mut total_freq: u64 = 0;
            let words: Vec<String> = sentence
                .split_whitespace()
                .map(|s| Util::clean_token(s, &GENERIC_WORD_GARBAGE_PATTERNS))
                .filter(|s| !s.is_empty())
                .collect();

            for word in &words {
                total_freq += data.freqs[word];
            }

            // NOTE: the idea here is to have a weighted penalty for the high word count.
            // The penalty is not just weighted, it's also exponential. Meaning the penalty gets exponentially higher the more words the sentence has.
            let word_count = words.len() as u64;
            // Average frequency divided by a word count penalty
            let avg_freq = total_freq / word_count;
            let penalty_factor = EXP_WORD_COUNT_PENALTY_FACTOR;
            let word_penalty = (word_count as f64).powf(penalty_factor); // Exponential penalty for length
            let score = (avg_freq as f64 / word_penalty) as u64;

            rankings.push(Rank::new(sentence.clone(), score));
        }

        rankings.sort_by(|a, b| a.score.cmp(&b.score));
        rankings
    }
}
