use ezql_core::{
    dialects::{Dialect, SqliteDialect},
    prelude::*,
};

fn main() {
    let table = Table {
        name: "test".to_string(),
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
    };

    println!("{}", table);

    println!("{}", SqliteDialect::create_table(true, table));
}
