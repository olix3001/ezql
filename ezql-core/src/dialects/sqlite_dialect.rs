use crate::components::query::Query;
use crate::dialects::Dialect;
use crate::types::EzqlType;

// ====< Dialect for SQLite >====
#[derive(Default)]
pub struct SqliteDialect {}

impl Dialect for SqliteDialect {
    // ====< Translate EzqlType to SQLite type >====
    fn translate_type(t: EzqlType) -> String {
        match t {
            EzqlType::Integer() => "INTEGER".to_string(),
            EzqlType::VarChar(len) => format!("VARCHAR({})", len),
            EzqlType::Boolean() => "BOOLEAN".to_string(),
            #[allow(unreachable_patterns)]
            _ => unimplemented!("Type {:?} is not implemented for SQLite dialect", t),
        }
    }

    // ====< Translate EzqlValue to SQLite value >====
    fn translate_value(v: crate::types::EzqlValue) -> String {
        match v {
            crate::types::EzqlValue::Integer(value) => value.to_string(),
            crate::types::EzqlValue::VarChar(value) => format!("'{}'", value),
            crate::types::EzqlValue::Boolean(value) => value.to_string().to_ascii_uppercase(),
            #[allow(unreachable_patterns)]
            _ => unimplemented!("Value {:?} is not implemented for SQLite dialect", v),
        }
    }

    // ====< Translate EzqlProperty to SQLite property >====
    fn translate_property(p: crate::components::column::ColumnProperty) -> String {
        match p {
            crate::components::column::ColumnProperty::PrimaryKey => "PRIMARY KEY".to_string(),
            crate::components::column::ColumnProperty::NotNull => "NOT NULL".to_string(),
            crate::components::column::ColumnProperty::Unique => "UNIQUE".to_string(),
            crate::components::column::ColumnProperty::Default(value) => {
                format!("DEFAULT {}", Self::translate_value(value))
            }
            #[allow(unreachable_patterns)]
            _ => unimplemented!("Property {:?} is not implemented for SQLite dialect", p),
        }
    }

    // ====< Create table >====
    fn create_table(if_not_exists: bool, table: crate::components::table::Table) -> Query {
        // Create table keyword
        let mut sql = format!(
            "CREATE TABLE {}{} (",
            if if_not_exists { "IF NOT EXISTS " } else { "" },
            table.name
        );

        // Add columns
        for (i, column) in table.columns.iter().enumerate() {
            sql.push_str(&format!(
                "{} {} {}",
                column.name,
                SqliteDialect::translate_type(column.data_type.clone()),
                column
                    .properties
                    .iter()
                    .map(|p| SqliteDialect::translate_property(p.clone()))
                    .collect::<Vec<String>>()
                    .join(" ")
            ));

            if i < table.columns.len() - 1 {
                sql.push_str(", ");
            }
        }

        // Close parentheses
        sql.push_str(");");

        // Return query
        Query::without_params(sql)
    }
}

// ====< Impl >====
