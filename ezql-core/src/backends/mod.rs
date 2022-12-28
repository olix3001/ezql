use crate::{
    dialects::Dialect,
    prelude::{EzqlModelTrait, EzqlValue, Table},
    queries::SelectQueryParams,
};

#[cfg(feature = "sqlite")]
pub mod sqlite_backend;

// ====< Backend trait >====
pub trait Backend<D>
where
    D: Dialect,
{
    // ====< Close connection >====
    fn close(self) -> Result<(), Box<dyn std::error::Error>>;

    // ====< Create table >====
    fn create_table(
        &self,
        if_not_exists: bool,
        table: Table,
    ) -> Result<(), Box<dyn std::error::Error>>;

    // ====< Drop table >====
    fn drop_table(&self, if_exists: bool, table: Table) -> Result<(), Box<dyn std::error::Error>>;

    // ====< Insert >====
    fn insert(
        &self,
        table: &Table,
        models: Vec<Vec<Option<EzqlValue>>>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    // ====< Select >====
    fn select(
        &self,
        table: &Table,
        query: SelectQueryParams,
    ) -> Result<Vec<Vec<Option<EzqlValue>>>, Box<dyn std::error::Error>>;
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

    // ====< Drop table >====
    fn drop_table<M>(&self, if_exists: bool) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait;

    // ====< Insert >====
    fn insert<M>(&self, models: &[&M]) -> Result<(), Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait;

    // ====< Select >====
    fn select<M>(&self, query: SelectQueryParams) -> Result<Vec<M>, Box<dyn std::error::Error>>
    where
        M: EzqlModelTrait;
}
