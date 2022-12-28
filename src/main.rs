use ezql_core::{
    prelude::*,
    queries::{OrderBy, SelectQueryParams, WhereClause},
    SqliteBackend,
};

#[derive(Debug)]
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
        columns: Some(vec!["id".to_string(), "is_active".to_string()]),
        where_clause: Some(WhereClause::Eq("name".to_string(), "John".into())),
        order_by: Some(OrderBy::Desc("id".to_string())),
        limit: Some(4),
        offset: None,
    };

    println!(
        "{:?}",
        backend.select::<User>(select_params.clone()).unwrap()
    );

    // Delete data
    println!(
        "Deleted {} rows",
        backend.delete::<User>(select_params).unwrap()
    );

    // Close connection
    ezql_core::Backend::close(backend).unwrap();
}
