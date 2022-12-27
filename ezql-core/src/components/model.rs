use crate::prelude::{EzqlValue, Table};

// ====< Trait for models >====
pub trait EzqlModelTrait {
    fn get_table() -> Table;
    fn as_column_values(&self) -> Vec<Option<EzqlValue>>;
}
