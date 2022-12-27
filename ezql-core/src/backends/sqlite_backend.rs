use rusqlite::Connection;

use crate::{
    dialects::{Dialect, SqliteDialect},
    prelude::{EzqlModelTrait, EzqlValue, Table},
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
    // ====< Create table >====
    fn create_table(
        &self,
        if_not_exists: bool,
        table: Table,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = SqliteDialect::create_table(if_not_exists, table);
        self.connection.execute(&query.sql, [])?;
        Ok(())
    }

    // ====< Insert >====
    fn insert(
        &self,
        table: Table,
        models: Vec<Vec<Option<EzqlValue>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = SqliteDialect::insert(table, models);
        self.connection.execute(
            &query.sql,
            rusqlite::params_from_iter(query.params.as_slice()),
        )?;
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
    // ====< Create table >====
    fn create_table<M>(&self, if_not_exists: bool) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait,
    {
        Backend::create_table(self, if_not_exists, M::get_table())
    }

    // ====< Insert >====
    fn insert<M>(&self, models: &[&M]) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait,
    {
        let table = M::get_table();
        let values = models
            .iter()
            .map(|m| m.as_column_values())
            .collect::<Vec<Vec<Option<EzqlValue>>>>();
        Backend::insert(self, table, values)
    }
}

// ====< Impl ToSql for EzqlValue >====
#[cfg(feature = "sqlite")]
impl rusqlite::types::ToSql for EzqlValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        match self {
            EzqlValue::Integer(i) => Ok(rusqlite::types::ToSqlOutput::from(*i)),
            EzqlValue::VarChar(s) => Ok(rusqlite::types::ToSqlOutput::from(s.as_str())),
            EzqlValue::Boolean(b) => Ok(rusqlite::types::ToSqlOutput::from(*b)),
        }
    }
}

// ========< TESTS >========
#[cfg(test)]
mod tests {
    use crate::{
        backends,
        prelude::{Column, ColumnProperty, EzqlType},
    };

    use super::*;

    struct User {
        id: Option<i32>,
        name: Option<String>,
        is_active: Option<bool>,
    }

    impl EzqlModelTrait for User {
        fn get_table() -> Table {
            Table {
                name: "users".to_string(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: EzqlType::Integer(),
                        properties: vec![ColumnProperty::PrimaryKey],
                    },
                    Column {
                        name: "name".to_string(),
                        data_type: EzqlType::VarChar(255),
                        properties: vec![ColumnProperty::NotNull],
                    },
                    Column {
                        name: "is_active".to_string(),
                        data_type: EzqlType::Boolean(),
                        properties: vec![ColumnProperty::default(false)],
                    },
                ],
            }
        }

        fn as_column_values(&self) -> Vec<Option<EzqlValue>> {
            vec![
                self.id.map(EzqlValue::Integer),
                self.name.as_ref().map(|v| EzqlValue::VarChar(v.clone())),
                self.is_active.map(EzqlValue::Boolean),
            ]
        }
    }

    #[test]
    fn test_create_dev_sqlite_backend() {
        let backend = SqliteBackend::new_in_memory();
        backends::ModelBackend::create_table::<User>(&backend, true).unwrap();
    }

    #[test]
    fn test_insert_sqlite_backend() {
        let backend = SqliteBackend::new_in_memory();
        let user = User {
            id: None,
            name: Some("John".to_string()),
            is_active: Some(true),
        };
        backends::ModelBackend::create_table::<User>(&backend, true).unwrap();
        backends::ModelBackend::insert::<User>(&backend, &[&user]).unwrap();
    }
}
