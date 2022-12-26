#[cfg(feature = "sqlite")]
pub mod sqlite_dialect;

pub trait Dialect: Sized {
    fn translate_type(t: crate::types::EzqlType) -> String;
}
