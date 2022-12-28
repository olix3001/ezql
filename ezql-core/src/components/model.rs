use crate::prelude::{EzqlValue, Table};

// ====< Trait for models >====
pub trait EzqlModelTrait {
    fn get_table() -> Table;
    fn as_column_values(&self) -> Vec<Option<EzqlValue>>;
    fn from_column_values(
        values: Vec<Option<EzqlValue>>,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
