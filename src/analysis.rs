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

    // FIXME: empty strings seem to get into the word frequencies somehow?
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
    // FIXME: Doesn't count the ratings properly. Arithmetic average does not take the number of words in a sentence into account, which is supposed to be the most important metric of easiness.
    fn rank(data: &RawData) -> Vec<Rank> {
        let mut rankings = vec![];

        for sentence in &data.sentences {
            let mut score: u64 = 0;
            let words: Vec<String> = sentence
                .split_whitespace()
                .map(|s| Util::clean_token(s, &GENERIC_WORD_GARBAGE_PATTERNS))
                .collect();

            for word in &words {
                score += data.freqs[word];
            }
            score /= words.len() as u64;

            rankings.push(Rank::new(sentence.clone(), score));
        }

        rankings.sort_by(|a, b| a.score.cmp(&b.score));
        rankings
    }
}
