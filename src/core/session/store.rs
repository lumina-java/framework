use async_trait::async_trait;
use tower_sessions::session::Record;
use tower_sessions::MemoryStore;
use tower_sessions::{session_store, SessionStore};
use tower_sessions_sqlx_store::{MySqlStore, PostgresStore, SqliteStore};

#[derive(Clone, Debug)]
pub enum LuminaSessionStore {
    Memory(MemoryStore),
    MySql(MySqlStore),
    Sqlite(SqliteStore),
    Postgres(PostgresStore),
}

#[async_trait]
impl SessionStore for LuminaSessionStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        match self {
            Self::Memory(s) => s.create(record).await,
            Self::MySql(s) => s.create(record).await,
            Self::Sqlite(s) => s.create(record).await,
            Self::Postgres(s) => s.create(record).await,
        }
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        match self {
            Self::Memory(s) => s.save(record).await,
            Self::MySql(s) => s.save(record).await,
            Self::Sqlite(s) => s.save(record).await,
            Self::Postgres(s) => s.save(record).await,
        }
    }

    async fn load(
        &self,
        session_id: &tower_sessions::session::Id,
    ) -> session_store::Result<Option<Record>> {
        match self {
            Self::Memory(s) => s.load(session_id).await,
            Self::MySql(s) => s.load(session_id).await,
            Self::Sqlite(s) => s.load(session_id).await,
            Self::Postgres(s) => s.load(session_id).await,
        }
    }

    async fn delete(&self, session_id: &tower_sessions::session::Id) -> session_store::Result<()> {
        match self {
            Self::Memory(s) => s.delete(session_id).await,
            Self::MySql(s) => s.delete(session_id).await,
            Self::Sqlite(s) => s.delete(session_id).await,
            Self::Postgres(s) => s.delete(session_id).await,
        }
    }
}
