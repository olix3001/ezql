use super::column::Column;

// ====< SQL table >====
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

// ====< Pretty print table >====
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Calculate the length of the longest column name
        let max_column_name_length = self
            .columns
            .iter()
            .map(|column| column.name.len())
            .max()
            .unwrap_or(0);
        let max_column_type_length = self
            .columns
            .iter()
            .map(|column| column.data_type.to_string().len())
            .max()
            .unwrap_or(0);
        let max_column_length = std::cmp::max(max_column_name_length, max_column_type_length);

        // Output string
        let mut output = String::new();

        // Table name
        output.push_str(&format!("TABLE: {}\n", self.name));
        // Column names separated by | and padded to the length of the longest column name
        for column in &self.columns {
            output.push_str(&format!(
                "| {:width$} ",
                column.name,
                width = max_column_length
            ));
        }
        output.push_str("|\n");

        // Column types separated by | and padded to the length of the longest column name
        for column in &self.columns {
            output.push_str(&format!(
                "| {:width$} ",
                column.data_type.to_string(),
                width = max_column_length
            ));
        }
        output.push_str("|\n");
        write!(f, "{}", output)
    }
}
