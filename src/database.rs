use crate::schema::frequencies::dsl as dslfreq;
use crate::schema::sentence_rankings::dsl as dslrank;
use crate::schema::{frequencies, sentence_rankings};
use anyhow::{Context, Result};
use diesel::dsl::count_star;
use diesel::{Insertable, Queryable};
/// This module is responsible for the SQLite database that stores and represents word frequencies and sentence rankings based on specific languages.
use diesel::{insert_into, prelude::*};
use diesel_migrations::MigrationHarness;

use crate::constants::MIGRATIONS;

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

pub struct Database {
    conn: SqliteConnection,
    frequencies: Vec<Frequency>,
    rankings: Vec<SentenceRanking>,
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
        })
    }

    pub fn insert_freqs(&mut self, new_freqs: &[Frequency]) -> Result<()> {
        insert_into(dslfreq::frequencies)
            .values(new_freqs)
            .execute(&mut self.conn)
            .map(|_| ())
            .context("Failed to insert frequencies")
    }

    pub fn insert_rankings(&mut self, new_rankings: &[SentenceRanking]) -> Result<()> {
        insert_into(dslrank::sentence_rankings)
            .values(new_rankings)
            .execute(&mut self.conn)
            .map(|_| ())
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
