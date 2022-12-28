use crate::{
    components::{
        column::ColumnProperty,
        query::{OrderBy, Query, SelectQueryParams, UpdateQueryParams, WhereClause},
        table::Table,
    },
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
    fn translate_order_by(order_by: OrderBy) -> String;

    // ====< Query translations >====
    fn translate_where_clause(where_clause: WhereClause) -> Query;

    // ====< Advanced translations >====
    fn create_table(if_not_exists: bool, table: Table) -> Query;
    fn drop_table(if_exists: bool, table: Table) -> Query;
    fn insert(table: &Table, models: Vec<Vec<Option<EzqlValue>>>) -> Query;
    fn select(table: &Table, query_params: SelectQueryParams) -> Query;
    fn delete(table: &Table, query_params: SelectQueryParams) -> Query;
    fn update(table: &Table, query_params: UpdateQueryParams) -> Query;
}
