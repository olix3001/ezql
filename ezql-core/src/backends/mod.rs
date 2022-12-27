use crate::{
    dialects::Dialect,
    prelude::{EzqlModelTrait, EzqlValue, Table},
};

#[cfg(feature = "sqlite")]
pub mod sqlite_backend;

// ====< Backend trait >====
pub trait Backend<D>
where
    D: Dialect,
{
    // ====< Create table >====
    fn create_table(
        &self,
        if_not_exists: bool,
        table: Table,
    ) -> Result<(), Box<dyn std::error::Error>>;

    // ====< Insert >====
    fn insert(
        &self,
        table: Table,
        models: Vec<Vec<Option<EzqlValue>>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

// ====< Model backend trait >====
pub trait ModelBackend<D>
where
    D: Dialect,
{
    // ====< Create table >====
    fn create_table<M>(&self, if_not_exists: bool) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait;

    // ====< Insert >====
    fn insert<M>(&self, models: &[&M]) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait;
}
