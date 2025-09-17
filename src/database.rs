use std::collections::HashMap;

use crate::analysis::SentenceRanker;
use crate::schema::frequencies::dsl as dslfreq;
use crate::schema::sentence_rankings::dsl as dslrank;
use crate::schema::{frequencies, sentence_rankings};
use anyhow::{Context, Result};
use diesel::dsl::count_star;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use diesel::{Insertable, Queryable};
/// This module is responsible for the SQLite database that stores and represents word frequencies and sentence rankings based on specific languages.
use diesel::{insert_into, prelude::*};
use diesel_migrations::MigrationHarness;

use crate::analysis::RawData;
use crate::constants::{DEFAULT_LANGUAGE, MIGRATIONS};

// TODO: [LATER] Develop the ability to sync database from/to some external "cloud" source for quick fetch on a different machine. Potentially copy into a cloud folder or push/pull to/from GitHub.

// TODO: Create a basic Database struct that can load up the two base models (sentence_rankings and frequencies) and start "talking" (inserting/querying) to them.
// TODO: start actually adding the frequencies and sentence rankings to the database after Sage runs through the specified text file.
// TODO: develop the ability to just fetch first N frequencies from the database.
// TODO: Develop the ability to just fetch first N sentence rankings from the database.

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = frequencies)]
pub struct Frequency {
    #[diesel(skip_insertion)]
    pub id: Option<i32>,
    pub word: String,
    pub frequency: i64,
    pub lang: String,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = sentence_rankings)]
pub struct SentenceRanking {
    #[diesel(skip_insertion)]
    pub id: Option<i32>,
    pub sentence: String,
    pub ranking: i64,
    pub lang: String,
}

pub struct DatabaseAdapter;
impl DatabaseAdapter {
    // NOTE: for now, I don't need an instance of the adapter, it's looking more like a collection of static methods (namespace). I'm still keeping an instance of it in the database though, for the future.
    pub fn new() -> Self {
        Self {}
    }

    pub fn freqs_to_db(conv: &HashMap<String, u64>) -> Vec<Frequency> {
        conv.iter()
            .map(|(word, freq)| Frequency {
                id: None,
                word: word.to_string(),
                frequency: *freq as i64,
                // FIXME(2): for now, language does not get forwarded from the DEFAULT_LANGUAGE env variable, it's just hardcoded in. Needs to be forwarded properly.
                lang: DEFAULT_LANGUAGE.to_owned(),
            })
            .collect()
    }

    pub fn raw_data_to_db_rankings(data: RawData) -> Vec<SentenceRanking> {
        // FIXME: Refactor? Should database adapter really be responsible for ranking new sentences, or does that responsibility actually lie elsewhere?
        let ranker = SentenceRanker::new(data);
        let ranks = ranker.rankings();
        ranks
            .iter()
            .map(|r| SentenceRanking {
                id: None,
                sentence: r.sentence.clone(),
                ranking: r.score as i64,
                // FIXME(2): [DUPLICATE] for now, language does not get forwarded from the DEFAULT_LANGUAGE env variable, it's just hardcoded in. Needs to be forwarded properly.
                lang: DEFAULT_LANGUAGE.to_string(),
            })
            .collect()
    }
}

// FIXME: Either make sure these unused fields get used, or remove them from the Database struct.
pub struct Database {
    conn: SqliteConnection,
    frequencies: Vec<Frequency>,
    rankings: Vec<SentenceRanking>,
    adapter: DatabaseAdapter,
}

impl Database {
    pub fn new(url: &str) -> Result<Self> {
        let mut conn =
            SqliteConnection::establish(url).context("while opening the SQLite database")?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("Running the SQLite Diesel migrations failed: `{e}`"))?;

        let frequencies: Vec<Frequency> = frequencies::table.load(&mut conn)?;
        let rankings: Vec<SentenceRanking> = sentence_rankings::table.load(&mut conn)?;

        Ok(Self {
            conn,
            frequencies,
            rankings,
            adapter: DatabaseAdapter::new(),
        })
    }

    pub fn insert_freqs(&mut self, new_freqs: &HashMap<String, u64>) -> Result<()> {
        let target_freqs = DatabaseAdapter::freqs_to_db(new_freqs);
        self.conn
            .transaction::<_, DieselError, _>(|conn| {
                for freq in target_freqs {
                    insert_into(dslfreq::frequencies)
                        .values(&freq)
                        .on_conflict((dslfreq::word, dslfreq::lang))
                        .do_update()
                        .set(
                            dslfreq::frequency
                                .eq(dslfreq::frequency + excluded(dslfreq::frequency)),
                        )
                        .execute(conn)?;
                }
                Ok(())
            })
            .context("Failed to insert frequencies")
    }

    pub fn insert_rankings(&mut self, raw_data: RawData) -> Result<()> {
        let new_rankings = DatabaseAdapter::raw_data_to_db_rankings(raw_data);
        self.conn
            .transaction::<_, DieselError, _>(|conn| {
                for ranking in new_rankings {
                    insert_into(dslrank::sentence_rankings)
                        .values(&ranking)
                        .on_conflict((dslrank::sentence, dslrank::lang))
                        .do_update()
                        .set(dslrank::ranking.eq(dslrank::ranking + excluded(dslrank::ranking)))
                        .execute(conn)?;
                }
                Ok(())
            })
            .context("Failed to insert sentence rankings")
    }

    pub fn top_freqs(&mut self, maybe_limit: Option<u32>) -> Result<Vec<Frequency>> {
        let limit: i64 = maybe_limit.map(|n| Ok(n as i64)).unwrap_or_else(|| {
            dslfreq::frequencies
                .select(count_star())
                .first(&mut self.conn)
        })?;
        dslfreq::frequencies
            .order(dslfreq::frequency.desc())
            .limit(limit)
            .load(&mut self.conn)
            .context("While fetching top frequencies from DB")
    }

    pub fn top_rankings(&mut self, maybe_limit: Option<u32>) -> Result<Vec<SentenceRanking>> {
        let limit: i64 = maybe_limit.map(|n| Ok(n as i64)).unwrap_or_else(|| {
            dslrank::sentence_rankings
                .select(count_star())
                .first(&mut self.conn)
        })?;
        dslrank::sentence_rankings
            .order(dslrank::ranking.desc())
            .limit(limit)
            .load(&mut self.conn)
            .context("While fetching top sentence rankings from DB")
    }
}
