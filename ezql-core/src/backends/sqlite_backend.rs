use rusqlite::Connection;

use crate::{
    dialects::{Dialect, SqliteDialect},
    prelude::{EzqlModelTrait, Table},
};

use super::{Backend, ModelBackend};

// ====< SQLite backend >====
#[cfg(feature = "sqlite")]
pub struct SqliteBackend {
    connection: rusqlite::Connection,
}

// ====< SQLite backend trait implementation >====
#[cfg(feature = "sqlite")]
impl Backend<SqliteDialect> for SqliteBackend {
    fn create_table(
        &self,
        if_not_exists: bool,
        table: Table,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = SqliteDialect::create_table(if_not_exists, table);
        self.connection.execute(&query.sql, [])?;
        Ok(())
    }
}

// ====< SQLite backend implementation >====
#[cfg(feature = "sqlite")]
impl SqliteBackend {
    pub fn new(path: &str) -> Self {
        Self {
            connection: Connection::open(path).unwrap(),
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            connection: Connection::open_in_memory().unwrap(),
        }
    }
}

// ====< SQlite model backend trait implementation >====
#[cfg(feature = "sqlite")]
impl ModelBackend<SqliteDialect> for SqliteBackend {
    fn create_table<M>(&self, if_not_exists: bool) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait,
    {
        Backend::create_table(self, if_not_exists, M::get_table())
    }
}
