use crate::{dialects::Dialect, prelude::Table};

#[cfg(feature = "sqlite")]
pub mod sqlite_backend;

// ====< Backend trait >====
pub trait Backend<D>: Sized
where
    D: Dialect,
{
    // ====< Create table >====
    fn create_table(
        &self,
        if_not_exists: bool,
        table: Table,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
