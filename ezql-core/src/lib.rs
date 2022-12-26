mod backends;
mod components;
pub mod dialects;
pub mod types;
mod utils;

pub mod prelude {
    pub use crate::backends::Backend;
    pub use crate::components::column::Column;
    pub use crate::components::column::ColumnProperty;
    pub use crate::components::table::Table;
    pub use crate::types::EzqlType;
    pub use crate::types::EzqlValue;
}

#[cfg(feature = "sqlite")]
pub use backends::sqlite_backend::SqliteBackend;
