use crate::{
    components::{column::ColumnProperty, query::Query, table::Table},
    prelude::EzqlValue,
    types::EzqlType,
};

#[cfg(feature = "sqlite")]
pub mod sqlite_dialect;
#[cfg(feature = "sqlite")]
pub use sqlite_dialect::SqliteDialect;

pub trait Dialect {
    // ====< Basic translations >====
    fn translate_type(t: EzqlType) -> String;
    fn translate_value(v: EzqlValue) -> String;
    fn translate_property(p: ColumnProperty) -> String;

    // ====< Advanced translations >====
    fn create_table(if_not_exists: bool, table: Table) -> Query;
}
