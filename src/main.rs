use ezql_core::{
    dialects::{Dialect, SqliteDialect},
    prelude::*,
    queries::{OrderBy, SelectQueryParams, WhereClause},
    SqliteBackend,
};

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

fn main() {
    let backend = SqliteBackend::new("test.db");

    println!("{}", User::get_table());

    // Create table
    backend.create_table::<User>(true).unwrap();

    // Insert data
    let user = User {
        id: None,
        name: Some("John".to_string()),
        is_active: Some(true),
    };
    let user2 = User {
        id: None,
        name: Some("Jane".to_string()),
        is_active: Some(false),
    };
    let user3 = User {
        id: None,
        name: Some("Jack".to_string()),
        is_active: None,
    };

    backend.insert(&[&user, &user2, &user3]).unwrap();

    // Select data
    let select_params = SelectQueryParams {
        columns: Some(vec!["id".to_string(), "name".to_string()]),
        where_clause: Some(WhereClause::And(vec![
            WhereClause::Eq("name".to_string(), "John".into()),
            WhereClause::Eq("is_active".to_string(), true.into()),
            WhereClause::Or(vec![
                WhereClause::Eq("name".to_string(), "Jane".into()),
                WhereClause::Eq("name".to_string(), "Jack".into()),
            ]),
        ])),
        order_by: Some(OrderBy::Desc("id".to_string())),
        limit: Some(4),
        offset: None,
    };

    println!(
        "{:?}",
        SqliteDialect::select(User::get_table(), select_params)
    );

    // Close connection
    ezql_core::Backend::close(backend).unwrap();
}
