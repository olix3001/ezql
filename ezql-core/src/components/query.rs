// ====< SQL query >====
#[derive(Debug, Clone)]
pub struct Query {
    pub sql: String,
    pub params: Vec<String>,
}

impl Query {
    pub fn new(sql: String, params: Vec<String>) -> Self {
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
