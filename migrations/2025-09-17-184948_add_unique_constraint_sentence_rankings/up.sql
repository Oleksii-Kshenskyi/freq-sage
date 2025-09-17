CREATE UNIQUE INDEX idx_sentence_rankings_unique ON sentence_rankings (sentence, lang);
CREATE UNIQUE INDEX idx_frequencies_unique ON frequencies (word, lang);