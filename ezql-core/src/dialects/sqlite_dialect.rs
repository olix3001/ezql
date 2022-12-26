use crate::dialects::Dialect;
use crate::types::EzqlType;

// ====< Dialect for SQLite >====
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
}
