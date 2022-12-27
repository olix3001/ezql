use ezql_core::{prelude::*, SqliteBackend};

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

    // Close connection
    ezql_core::Backend::close(backend).unwrap();
}
