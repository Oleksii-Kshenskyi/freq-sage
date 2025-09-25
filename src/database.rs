use std::cmp::Ordering;
use std::collections::HashMap;

use anyhow::{Context, Result};
use blake3::Hasher;
use redb::{Database, ReadableTableMetadata, TableDefinition};
use redb::{ReadableDatabase, ReadableTable};
use redb_derive::Value;

use crate::analysis::RawData;
/// This module is responsible for the redb key/value database that stores and represents word frequencies and sentence rankings based on specific languages.
use crate::analysis::SentenceRanker;
use crate::constants::REDB_LAYOUT_VERSION;
use crate::util::Util;

// TODO: [LATER] Develop the ability to sync database from/to some external "cloud" source for quick fetch on a different machine. Potentially copy into a cloud folder or push/pull to/from GitHub.

// TODO: develop the ability to just fetch first N frequencies from the database.
// TODO: Develop the ability to just fetch first N sentence rankings from the database.

// Primary tables in the DB: words by word hash and sentences by sentence hash
const FREQUENCIES: TableDefinition<[u8; 32], FrequencyDoc> = TableDefinition::new("frequencies");
const SENTENCES: TableDefinition<[u8; 32], SentenceDoc> = TableDefinition::new("sentences");

// "Secondary" tables in the DB: search indexes for common queries like "top N frequencies" etc.

/// Word frequency index: for finding top N frequencies.
/// Value is empty, because it stores all the necessary info (the frequency and the word hash) in the key.
const WORD_FREQ_INDEX: TableDefinition<(u64, [u8; 32]), ()> = TableDefinition::new("freq_index");
const SENTENCE_RANK_INDEX: TableDefinition<(u64, [u8; 32]), ()> =
    TableDefinition::new("rankings_index");

/// System config meta-table that for now only includes one record in the key, the current database version.
const SYSTEM: TableDefinition<u32, ()> = TableDefinition::new("system_table");

#[derive(Debug, Value)]
pub struct SentenceDoc {
    pub raw: String,
    pub rating: u64,
}

impl SentenceDoc {
    pub fn new(sentence: String, ranking: u64) -> Self {
        SentenceDoc {
            raw: sentence,
            rating: ranking,
        }
    }
}

#[derive(Debug, Value)]
pub struct FrequencyDoc {
    pub word: String,
    pub freq: u64,
}

impl FrequencyDoc {
    pub fn new(word: String, freq: u64) -> Self {
        Self { word, freq }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        Util::hash_word(&self.word, &mut hasher);
        *hasher.finalize().as_bytes()
    }
}

pub struct SageDatabase {
    db: Database,
    lang: String,
    version_inconsistency: bool,
}

impl SageDatabase {
    pub fn new(lang: &str) -> Result<Self> {
        // NOTE: Databases are called according to the language they're storing sentences/frequencies in, like English.redb, Finnish.redb etc.
        let filename = format!("{}.redb", lang);
        let filepath = std::path::Path::new(&filename);
        let db = if filepath.exists() {
            redb::Database::open(filepath)?
        } else {
            redb::Database::create(filepath)?
        };
        let version_inconsistency: bool;

        {
            let wtx = db.begin_write()?;

            let _ = wtx.open_table(FREQUENCIES)?;
            let _ = wtx.open_table(SENTENCES)?;
            let _ = wtx.open_table(WORD_FREQ_INDEX)?;
            let _ = wtx.open_table(SENTENCE_RANK_INDEX)?;

            let _ = wtx.open_table(SYSTEM)?;

            wtx.commit()?;

            version_inconsistency = Self::ensure_version_consistency(&db)?;
        }

        Ok(Self {
            db,
            lang: lang.to_owned(),
            version_inconsistency,
        })
    }

    pub fn insert_freqs(&mut self, data: &RawData) -> Result<()> {
        let wtx = self.db.begin_write()?;
        {
            let mut freq_table = wtx.open_table(FREQUENCIES)?;
            let mut index_table = wtx.open_table(WORD_FREQ_INDEX)?;

            for (word, freq) in &data.freqs {
                let mut doc = FrequencyDoc::new(word.clone(), 0);
                let hash = doc.hash();

                let dbval = freq_table.get(&hash)?.map(|v| v.value().freq).unwrap_or(0);
                let new_freq = dbval.saturating_add(*freq);
                doc.freq = new_freq;
                freq_table.insert(&hash, doc)?;

                if dbval != 0 {
                    index_table.remove(&(dbval, hash))?;
                }
                index_table.insert(&(new_freq, hash), ())?;
            }
        }
        wtx.commit()?;
        Ok(())
    }

    pub fn insert_rankings(
        &mut self,
        db_freqs: HashMap<String, u64>,
        sentences: Vec<String>,
    ) -> Result<()> {
        let data = RawData::from_preexisting_data(db_freqs, sentences, self.lang.clone());
        let ranker = SentenceRanker::new(&data);
        let new_rankings = ranker.rankings();
        let wtx = self.db.begin_write()?;
        {
            let mut rank_table = wtx.open_table(SENTENCES)?;
            let mut index_table = wtx.open_table(SENTENCE_RANK_INDEX)?;

            for ranking in new_rankings {
                let mut doc = SentenceDoc::new(ranking.sentence.clone(), ranking.score);
                let hash = Util::hash_words(&ranking.words);
                let dbval = rank_table
                    .get_mut(hash)?
                    .map(|guard| guard.value().rating)
                    .unwrap_or(0);
                let new_val = if dbval == 0 {
                    doc.rating
                } else {
                    SentenceRanker::rank_sentence(&doc.raw, &data.freqs, None).expect("SageDatabase::insert_rankings(): sentence rank is supposed to always be Some here, but it's None?").score
                };
                doc.rating = new_val;
                rank_table.insert(hash, doc)?;

                if dbval != 0 {
                    index_table.remove(&(dbval, hash))?;
                }
                index_table.insert(&(new_val, hash), ())?;
            }
        }
        wtx.commit()?;

        Ok(())
    }

    pub fn freqs_of_words(&self, words: &[String]) -> Result<HashMap<String, u64>> {
        let rtx = self.db.begin_read()?;
        let freqs = rtx.open_table(FREQUENCIES)?;
        words.iter().map(|w|{
            let doc = FrequencyDoc::new(w.clone(), 0);
            match freqs.get(&doc.hash())? {
                Some(doc_guard) => Ok((doc.word, doc_guard.value().freq)),
                None => panic!("{}", &format!("UNREACHABLE: only words whose records already exist should be looked up in this method, and yet the word `{}` did not exist...", w)),
            }
        }).collect::<Result<HashMap<_,_>, _>>()
    }

    // REFACTOR: [???] can top_freqs() and top_rankings() be merged into a single function?
    pub fn top_freqs(&mut self, maybe_limit: Option<u32>) -> Result<Vec<FrequencyDoc>> {
        self.ensure_index_consistency(WORD_FREQ_INDEX, FREQUENCIES, Self::build_freq_index)?;

        let rtx = self.db.begin_read()?;
        let index_table = rtx.open_table(WORD_FREQ_INDEX)?;
        let primary_freqs_table = rtx.open_table(FREQUENCIES)?;

        let limit = maybe_limit.map(|l| l as usize).unwrap_or(usize::MAX);
        let mut result = Vec::with_capacity(maybe_limit.unwrap_or(0) as usize);

        for row in index_table.iter()?.take(limit) {
            let (key_guard, _) = row?;
            let (freq, hash) = key_guard.value();
            let word = primary_freqs_table
                .get(&hash)?
                .map(|doc_guard| doc_guard.value().word)
                .context("SageDatabase::top_freqs(): UNREACHABLE: hash from index table is not in the primary frequencies table???")?;
            result.push(FrequencyDoc::new(word, freq));
        }

        Ok(result)
    }

    pub fn top_rankings(&mut self, maybe_limit: Option<u32>) -> Result<Vec<SentenceDoc>> {
        self.ensure_index_consistency(SENTENCE_RANK_INDEX, SENTENCES, Self::build_rankings_index)?;

        let rtx = self.db.begin_read()?;
        let index_table = rtx.open_table(SENTENCE_RANK_INDEX)?;
        let primary_table = rtx.open_table(SENTENCES)?;

        let limit = maybe_limit.map(|l| l as usize).unwrap_or(usize::MAX);
        let mut result = Vec::with_capacity(maybe_limit.unwrap_or(0) as usize);

        for row in index_table.iter()?.take(limit) {
            let (key_guard, _) = row?;
            let (ranking, hash) = key_guard.value();
            let sentence = primary_table.get(&hash)?.map(|guard| guard.value().raw).context("SageDatabase::top_rankings(): UNREACHABLE: hash from index table is not in the primary sentences table???")?;
            result.push(SentenceDoc::new(sentence, ranking));
        }

        Ok(result)
    }

    fn version(db: &Database) -> Result<u32> {
        let rtx = db.begin_read()?;
        let systb = rtx.open_table(SYSTEM)?;
        if systb.len()? > 1 {
            panic!(
                "SageDatabase::ensure_version_consistency(): SYSTEM table has more than one record, but is supposed to only have one, where all the config variables are stored. This may mean that the data in the DB is inconsistent, or something went horribly wrong."
            );
        }
        Ok(systb
            .first()?
            .map(|(ver_guard, _)| ver_guard.value())
            .unwrap_or(0))
    }

    /// Static method that compares the version constant in the code and the version number in the database.
    /// @param db - the database to perform the check on.
    /// @returns - a Result<bool>. The result part is a possible IO error while reading from the DB. The bool part: true if a version inconsistency has been SPOTTED, false if the version is consistent between the code and the DB.
    fn ensure_version_consistency(db: &Database) -> Result<bool> {
        let current_version = Self::version(db)?;

        match current_version.cmp(&(REDB_LAYOUT_VERSION).into()) {
            // NOTE: if we've extracted a certain version from the DB, and it's exactly equal to the current DB layout version in our code, that means the version is up-to-date and we can safely return and do nothing.
            Ordering::Equal => Ok(false),
            Ordering::Less => {
                let wtx = db.begin_write()?;
                {
                    let mut systb = wtx.open_table(SYSTEM)?;
                    if current_version != 0 {
                        systb.remove(current_version)?;
                    }
                    systb.insert(REDB_LAYOUT_VERSION as u32, ())?;
                }
                wtx.commit()?;
                Ok(true)
            }
            Ordering::Greater => panic!(
                "SageDatabase::ensure_version_consistency(): the DB version in the code is smaller than the version in the database itself. This means an inconsistency between the code and the DB, this needs to be investigated separately - it shouldn't happen if FreQ Sage is behaving properly."
            ),
        }
    }

    /// Rebuilds an index for a primary table if one of these conditions is true:
    /// - Index table doesn't exist;
    /// - DB init noticed a version inconsistency between code and DB;
    /// - Index table and primary table lengths are not equal.
    ///   @returns - Ok(()) if all went well, Err(_) if IO errors.
    fn ensure_index_consistency<KI, VI, KP, VP, F>(
        &mut self,
        index_table_def: TableDefinition<KI, VI>,
        primary_table_def: TableDefinition<KP, VP>,
        index_build_func: F,
    ) -> Result<()>
    where
        KI: redb::Key,
        KP: redb::Key,
        VI: redb::Value,
        VP: redb::Value,
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let rtx = self.db.begin_read()?;
        let need_rebuild = match rtx.open_table(index_table_def) {
            Ok(index_table) => {
                if self.version_inconsistency {
                    true
                } else {
                    let freq_table = rtx.open_table(primary_table_def)?;
                    index_table.len()? != freq_table.len()?
                }
            }
            Err(_) => true,
        };
        if need_rebuild {
            index_build_func(self)
        } else {
            Ok(())
        }
    }

    // REFACTOR: [???] can build_freq_index() and build_rankings_index() be merged into a single function?
    fn build_freq_index(&mut self) -> Result<()> {
        let wtx = self.db.begin_write()?;
        {
            let primary_table = wtx.open_table(FREQUENCIES)?;
            wtx.delete_table(WORD_FREQ_INDEX)?;
            let mut index_table = wtx.open_table(WORD_FREQ_INDEX)?;
            for primary_pair in primary_table.iter()? {
                let (hash_guard, freq_doc_guard) = primary_pair?;
                let hash = hash_guard.value();
                let doc_freq = freq_doc_guard.value().freq;
                index_table.insert((doc_freq, hash), ())?;
            }
        }
        wtx.commit()?;

        Ok(())
    }

    fn build_rankings_index(&mut self) -> Result<()> {
        let wtx = self.db.begin_write()?;
        {
            let primary_table = wtx.open_table(SENTENCES)?;
            wtx.delete_table(SENTENCE_RANK_INDEX)?;
            let mut index_table = wtx.open_table(SENTENCE_RANK_INDEX)?;
            for primary_pair in primary_table.iter()? {
                let (hash_guard, sentence_doc_guard) = primary_pair?;
                let hash = hash_guard.value();
                let ranking = sentence_doc_guard.value().rating;
                index_table.insert((ranking, hash), ())?;
            }
        }
        wtx.commit()?;

        Ok(())
    }
}
