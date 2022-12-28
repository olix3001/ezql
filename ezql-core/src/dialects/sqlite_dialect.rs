use crate::components::column::ColumnProperty::Default;
use crate::components::query::{OrderBy, Query, SelectQueryParams, WhereClause};
use crate::components::table::Table;
use crate::dialects::Dialect;
use crate::types::{EzqlType, EzqlValue};

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
    fn translate_value(v: EzqlValue) -> String {
        match v {
            EzqlValue::Integer(value) => value.to_string(),
            EzqlValue::VarChar(value) => format!("'{}'", value),
            EzqlValue::Boolean(value) => value.to_string().to_ascii_uppercase(),
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
            Default(value) => {
                format!("DEFAULT {}", Self::translate_value(value))
            }
            #[allow(unreachable_patterns)]
            _ => unimplemented!("Property {:?} is not implemented for SQLite dialect", p),
        }
    }

    // ====< Translate WhereClause to SQLite WHERE clause >====
    fn translate_where_clause(where_clause: WhereClause) -> Query {
        // Create empty query
        let mut query = Query::empty();
        // Add clause
        match where_clause {
            WhereClause::And(clauses) => {
                for (i, clause) in clauses.iter().enumerate() {
                    let q = Self::translate_where_clause(clause.clone());
                    query.params.extend(q.params);
                    query.sql.push_str(&format!("({})", q.sql));

                    if i < clauses.len() - 1 {
                        query.sql.push_str(" AND ");
                    }
                }
            }
            WhereClause::Or(clauses) => {
                for (i, clause) in clauses.iter().enumerate() {
                    let q = Self::translate_where_clause(clause.clone());
                    query.params.extend(q.params);
                    query.sql.push_str(&format!("({})", q.sql));

                    if i < clauses.len() - 1 {
                        query.sql.push_str(" OR ");
                    }
                }
            }
            WhereClause::Eq(column, value) => {
                query.params.push(value);
                query.sql = format!("{} = ?", column);
            }
            WhereClause::Ne(column, value) => {
                query.params.push(value);
                query.sql = format!("{} != ?", column);
            }
            WhereClause::Gt(column, value) => {
                query.params.push(value);
                query.sql = format!("{} > ?", column);
            }
            WhereClause::Ge(column, value) => {
                query.params.push(value);
                query.sql = format!("{} >= ?", column);
            }
            WhereClause::Lt(column, value) => {
                query.params.push(value);
                query.sql = format!("{} < ?", column);
            }
            WhereClause::Le(column, value) => {
                query.params.push(value);
                query.sql = format!("{} <= ?", column);
            }
            WhereClause::Like(column, value) => {
                query.params.push(value);
                query.sql = format!("{} LIKE ?", column);
            }
            WhereClause::Not(clause) => {
                query.sql = format!("NOT {}", Self::translate_where_clause(*clause));
            }
            WhereClause::IsNull(column) => {
                query.sql = format!("{} IS NULL", column);
            }
            WhereClause::IsNotNull(column) => {
                query.sql = format!("{} IS NOT NULL", column);
            }
            WhereClause::In(column, values) => {
                query.params.extend(values.clone());
                query.sql = format!("{} IN ({})", column, "?,".repeat(values.len() - 1));
            }
            WhereClause::NotIn(column, values) => {
                query.params.extend(values.clone());
                query.sql = format!("{} NOT IN ({})", column, "?,".repeat(values.len() - 1));
            }

            WhereClause::All => {
                query.sql = "1 = 1".to_string();
            }

            #[allow(unreachable_patterns)]
            _ => unimplemented!(
                "WhereClause {:?} is not implemented for SQLite dialect",
                where_clause
            ),
        }

        query
    }

    // ====< Translate OrderBy to SQLite ORDER BY clause >====
    fn translate_order_by(order_by: OrderBy) -> String {
        match order_by {
            OrderBy::Asc(column) => format!("{} ASC", column),
            OrderBy::Desc(column) => format!("{} DESC", column),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(
                "OrderBy {:?} is not implemented for SQLite dialect",
                order_by
            ),
        }
    }

    // ====< Create table >====
    fn create_table(if_not_exists: bool, table: Table) -> Query {
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

    // ====< Drop table >====
    fn drop_table(if_exists: bool, table: Table) -> Query {
        // Create drop keyword
        let sql = format!(
            "DROP TABLE {}{};",
            if if_exists { "IF EXISTS " } else { "" },
            table.name
        );

        // Return query
        Query::without_params(sql)
    }

    // ====< Insert into table >====
    fn insert(table: &Table, models: Vec<Vec<Option<EzqlValue>>>) -> Query {
        // Create insert keyword
        let mut sql = format!("INSERT INTO {} (", table.name);

        // Create params
        let mut params = Vec::new();

        // Add columns
        for (i, column) in table.columns.iter().enumerate() {
            // If column is primary key, skip it
            if column.is_primary_key() {
                continue;
            }

            // If column is not specified, skip it
            sql.push_str(&column.name);

            if i < table.columns.len() - 1 {
                sql.push_str(", ");
            }
        }

        // Add values
        sql.push_str(") VALUES (");
        for model in models.iter() {
            for (i, value) in model.iter().enumerate() {
                // If value is not specified NULL or DEFAULT
                if value.is_none() {
                    // If column is primary key, skip it
                    if table.columns[i].is_primary_key() {
                        continue;
                    }

                    // If column has default value, use it
                    if table.columns[i].has_default() {
                        sql.push_str(&SqliteDialect::translate_value(
                            table.columns[i].get_default().unwrap(),
                        ));
                    } else {
                        sql.push_str("NULL");
                    }

                    if i < model.len() - 1 {
                        sql.push_str(", ");
                    }

                    continue;
                }

                // Add value to params
                params.push(value.clone().unwrap());

                // Add value to query
                sql.push('?');

                if i < model.len() - 1 {
                    sql.push_str(", ");
                }
            }

            sql.push_str("), (");
        }

        // Remove empty parentheses and comma
        sql.pop();
        sql.pop();
        sql.pop();

        // End query with semicolon
        sql.push(';');

        // Return query
        Query::new(sql, params)
    }

    // ====< Select from table >====
    fn select(table: &Table, query_params: SelectQueryParams) -> Query {
        // Create select keyword
        let mut sql = "SELECT ".to_string();

        // Create params
        let mut params = Vec::new();

        // Add columns from select query params
        if let Some(select_columns) = query_params.columns {
            sql.push_str(
                &select_columns
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        } else {
            sql.push('*');
        }

        // Add from keyword
        sql.push_str(&format!(" FROM {}", table.name));

        // Add where clause
        if let Some(where_clause) = query_params.where_clause {
            let where_clause = SqliteDialect::translate_where_clause(where_clause);
            params.extend(where_clause.params);
            sql.push_str(&format!(" WHERE {}", where_clause.sql));
        }

        // Add order by clause
        if let Some(order_by) = query_params.order_by {
            sql.push_str(&format!(
                " ORDER BY {}",
                SqliteDialect::translate_order_by(order_by)
            ));
        }

        // Add limit clause
        if let Some(limit) = query_params.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        // Add offset clause
        if let Some(offset) = query_params.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        // End query with semicolon
        sql.push(';');

        // Return query
        Query::new(sql, params)
    }

    // ====< Delete from table >====
    fn delete(table: &Table, query_params: SelectQueryParams) -> Query {
        // Create delete keyword
        let mut sql = format!("DELETE FROM {}", table.name);

        // Create params
        let mut params = Vec::new();

        // Add where clause
        if let Some(where_clause) = query_params.where_clause {
            let where_clause = SqliteDialect::translate_where_clause(where_clause);
            params.extend(where_clause.params);
            sql.push_str(&format!(" WHERE {}", where_clause.sql));
        }

        // Ignore order by, limit and offset

        // End query with semicolon
        sql.push(';');

        // Return query
        Query::new(sql, params)
    }
}

// ====< Impl >====
