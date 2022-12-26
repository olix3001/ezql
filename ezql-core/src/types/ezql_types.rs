// Macro to generate types
macro_rules! create_types {
    ($($name:ident $argT:tt => $argV:tt),*,) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum EzqlType {
            $($name $argT),*
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum EzqlValue {
            $($name $argV),*
        }
    }
}

// ====< Types and trait >====
// TODO: Add more types
create_types! {
    Integer () => (i32),
    VarChar (usize) => (String),
    Boolean () => (bool),
}

// Trait for converting Rust types to Ezql types
pub trait HasEzqlType<T> {
    fn from_rust_type() -> EzqlType;
    fn from_rust_value(_: T) -> EzqlValue;
}
