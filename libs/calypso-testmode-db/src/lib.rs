//! Test-mode database access for Calypso.
//!
//! Wraps the SQLite database that stores test profiles and their scheduled CAN
//! messages, exposing the diesel models and a small connection/query API so
//! callers never have to touch diesel directly.

use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};

pub mod models;
mod schema;

pub use models::{TestCanMessageEntry, TestProfile};

/// Errors that can occur while connecting to or querying the test database.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The `DATABASE_URL` environment variable was not set.
    #[error("DATABASE_URL environment variable is not set: {0}")]
    MissingDatabaseUrl(#[from] std::env::VarError),

    /// Establishing the SQLite connection failed.
    #[error("could not connect to test database at {url}")]
    Connection {
        url: String,
        #[source]
        source: diesel::ConnectionError,
    },

    /// No profile with the requested name exists.
    #[error("test profile '{0}' not found")]
    ProfileNotFound(String),

    /// A query against the database failed.
    #[error(transparent)]
    Query(#[from] diesel::result::Error),
}

/// Result alias for this crate's fallible operations.
pub type Result<T> = std::result::Result<T, Error>;

/// A connection to the test-mode SQLite database.
pub struct TestModeDb {
    conn: SqliteConnection,
}

impl TestModeDb {
    /// Connect to the SQLite database at the given URL/path.
    pub fn connect(database_url: &str) -> Result<Self> {
        let conn =
            SqliteConnection::establish(database_url).map_err(|source| Error::Connection {
                url: database_url.to_owned(),
                source,
            })?;
        Ok(Self { conn })
    }

    /// Connect using the `DATABASE_URL` environment variable, loading a `.env`
    /// file first if one is present.
    pub fn connect_from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")?;
        Self::connect(&database_url)
    }

    /// Look up a test profile by its unique name.
    pub fn find_profile_by_name(&mut self, profile_name: &str) -> Result<Option<TestProfile>> {
        use schema::test_profile::dsl::{name, test_profile};

        Ok(test_profile
            .filter(name.eq(profile_name))
            .select(TestProfile::as_select())
            .first(&mut self.conn)
            .optional()?)
    }

    /// Load every CAN message belonging to the named profile, ordered by
    /// ascending offset.
    ///
    /// Returns [`Error::ProfileNotFound`] if no profile with that name exists.
    pub fn load_profile_messages(
        &mut self,
        profile_name: &str,
    ) -> Result<Vec<TestCanMessageEntry>> {
        use schema::can_message::dsl::{can_message, offset_ms, profile_id};

        let profile = self
            .find_profile_by_name(profile_name)?
            .ok_or_else(|| Error::ProfileNotFound(profile_name.to_owned()))?;

        Ok(can_message
            .filter(profile_id.eq(profile.id))
            .select(TestCanMessageEntry::as_select())
            .order(offset_ms.asc())
            .load(&mut self.conn)?)
    }
}
