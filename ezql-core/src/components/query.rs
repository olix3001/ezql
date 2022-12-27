use crate::prelude::{EzqlValue, Table};

// ====< SQL query >====
#[derive(Debug, Clone)]
pub struct Query {
    pub sql: String,
    pub params: Vec<EzqlValue>,
}

impl Query {
    pub fn new(sql: String, params: Vec<EzqlValue>) -> Self {
        Self { sql, params }
    }
    pub fn without_params(sql: String) -> Self {
        Self::new(sql, vec![])
    }
}

// ====< Pretty print query >====
impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.sql)
    }
}

// ====< Select query parameters >====
#[derive(Debug, Clone)]
pub struct SelectQueryParams {
    pub table: Table,
    pub columns: Vec<String>,
    pub where_clause: Option<WhereClause>,
    pub order_by: Option<OrderBy>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// ====< Where clause >====
#[derive(Debug, Clone)]
pub enum WhereClause {
    And(Vec<WhereClause>),
    Or(Vec<WhereClause>),
    Eq(String, EzqlValue),
    Ne(String, EzqlValue),
    Gt(String, EzqlValue),
    Ge(String, EzqlValue),
    Lt(String, EzqlValue),
    Le(String, EzqlValue),
    Like(String, EzqlValue),
    Not(Box<WhereClause>),
    IsNull(String),
    IsNotNull(String),
    In(String, Vec<EzqlValue>),
    NotIn(String, Vec<EzqlValue>),
}

// ====< Order by >====
#[derive(Debug, Clone)]
pub struct OrderBy {
    pub column: String,
    pub order: Order,
}

// ====< Order >====
#[derive(Debug, Clone)]
pub enum Order {
    Asc,
    Desc,
}
