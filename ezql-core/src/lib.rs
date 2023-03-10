mod backends;
mod components;
pub mod dialects;
pub mod types;
mod utils;

pub use crate::backends::Backend;

pub mod queries {
    pub use crate::components::query::OrderBy;
    pub use crate::components::query::Query;
    pub use crate::components::query::SelectQueryParams;
    pub use crate::components::query::UpdateQueryParams;
    pub use crate::components::query::WhereClause;
}

pub mod prelude {
    pub use crate::backends::ModelBackend;
    pub use crate::components::column::Column;
    pub use crate::components::column::ColumnProperty;
    pub use crate::components::model::EzqlModelTrait;
    pub use crate::components::table::Table;
    pub use crate::types::EzqlType;
    pub use crate::types::EzqlValue;
}

#[cfg(feature = "sqlite")]
pub use backends::sqlite_backend::SqliteBackend;
