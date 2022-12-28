use rusqlite::Connection;

use crate::{
    dialects::{Dialect, SqliteDialect},
    prelude::{EzqlModelTrait, EzqlValue, Table},
    queries::SelectQueryParams,
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
    // ====< Close connection >====
    fn close(self) -> Result<(), Box<dyn std::error::Error>> {
        if self.connection.close().is_err() {
            return Err("Failed to close connection".into());
        }
        Ok(())
    }

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

    // ====< Drop table >====
    fn drop_table(&self, if_exists: bool, table: Table) -> Result<(), Box<dyn std::error::Error>> {
        let query = SqliteDialect::drop_table(if_exists, table);
        self.connection.execute(&query.sql, [])?;
        Ok(())
    }

    // ====< Insert >====
    fn insert(
        &self,
        table: &Table,
        models: Vec<Vec<Option<EzqlValue>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = SqliteDialect::insert(table, models);
        self.connection.execute(
            &query.sql,
            rusqlite::params_from_iter(query.params.as_slice()),
        )?;
        Ok(())
    }

    // ====< Select >====
    fn select(
        &self,
        table: &Table,
        query: SelectQueryParams,
    ) -> Result<Vec<Vec<Option<EzqlValue>>>, Box<dyn std::error::Error>> {
        let select_query = SqliteDialect::select(table, query.clone());
        let mut stmt = self.connection.prepare(&select_query.sql)?;
        let mut rows = stmt.query(rusqlite::params_from_iter(select_query.params.as_slice()))?;
        let mut result = Vec::new();
        let has_columns = query.columns.is_some();
        while let Some(row) = rows.next()? {
            let mut row_result = Vec::new();
            let mut idx = 0;
            for i in 0..table.columns.len() {
                // skip if column is not selected
                if has_columns
                    && !query
                        .columns
                        .as_ref()
                        .unwrap()
                        .contains(&table.columns[i].name)
                {
                    row_result.push(None);
                    continue;
                }
                // Select value
                row_result.push(row.get(idx)?);
                idx += 1;
            }
            result.push(row_result);
        }
        Ok(result)
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

    // ====< Drop table >====
    fn drop_table<M>(&self, if_exists: bool) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait,
    {
        Backend::drop_table(self, if_exists, M::get_table())
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
        Backend::insert(self, &table, values)
    }

    // ====< Select >====
    fn select<M>(&self, query: SelectQueryParams) -> Result<Vec<M>, Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait,
    {
        let table = M::get_table();
        let values = Backend::select(self, &table, query)?;
        let mut result = Vec::new();
        for value in values {
            println!("{:?}", value);
            result.push(M::from_column_values(value)?);
        }
        Ok(result)
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
            EzqlValue::Null() => Ok(rusqlite::types::ToSqlOutput::from(rusqlite::types::Null)),
        }
    }
}

// ====< Impl FromSql for EzqlValue >====
#[cfg(feature = "sqlite")]
impl rusqlite::types::FromSql for EzqlValue {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Integer(i) => Ok(EzqlValue::from(i as i32)),
            rusqlite::types::ValueRef::Text(s) => {
                Ok(EzqlValue::from(String::from_utf8(s.to_vec()).unwrap()))
            }
            rusqlite::types::ValueRef::Null => Ok(EzqlValue::Null()),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
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

        fn from_column_values(
            values: Vec<Option<EzqlValue>>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self {
                id: values[0].as_ref().map(|v| v.into()),
                name: values[1].as_ref().map(|v| v.into()),
                is_active: values[2].as_ref().map(|v| v.into()),
            })
        }
    }

    #[test]
    fn test_create_dev_sqlite_backend() {
        let backend = SqliteBackend::new_in_memory();
        backends::ModelBackend::create_table::<User>(&backend, true).unwrap();
    }

    #[test]
    fn test_drop_sqlite_backend() {
        let backend = SqliteBackend::new_in_memory();
        backends::ModelBackend::create_table::<User>(&backend, true).unwrap();
        backends::ModelBackend::drop_table::<User>(&backend, true).unwrap();

        // Insert should fail because table does not exist
        let user = User {
            id: None,
            name: Some("John".to_string()),
            is_active: Some(true),
        };

        assert!(backends::ModelBackend::insert::<User>(&backend, &[&user]).is_err());
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

    #[test]
    fn test_select_sqlite_backend() {
        let backend = SqliteBackend::new_in_memory();
        let user = User {
            id: None,
            name: Some("John".to_string()),
            is_active: Some(true),
        };
        backends::ModelBackend::create_table::<User>(&backend, true).unwrap();
        backends::ModelBackend::insert::<User>(&backend, &[&user]).unwrap();
        let users = backends::ModelBackend::select::<User>(
            &backend,
            SelectQueryParams {
                columns: Some(vec!["name".to_string()]),
                where_clause: None,
                order_by: None,
                limit: None,
                offset: None,
            },
        )
        .unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, Some("John".to_string()));
    }
}
