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
}

fn main() {
    let backend = SqliteBackend::new("test.db");

    println!("{}", User::get_table());

    backend.create_table::<User>(false).unwrap();
}
