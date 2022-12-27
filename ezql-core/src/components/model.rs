use crate::prelude::Table;

// ====< Trait for models >====
pub trait EzqlModelTrait {
    fn get_table() -> Table;
}
