use super::ezql_types::{EzqlType, EzqlValue, HasEzqlType};

// ====< Macro for generating type mappings >====
#[macro_export]
macro_rules! impl_ezql_types {
    ($($type:ty => $ezql_type:ident ($($argT:expr),*)),*,) => {
        $(
            impl HasEzqlType<$type> for EzqlType {
                fn from_rust_type() -> EzqlType {
                    EzqlType::$ezql_type($($argT),*)
                }

                fn from_rust_value(value: $type) -> EzqlValue {
                    EzqlValue::$ezql_type(value.into())
                }
            }
        )*
    };
}

// ====< Rust type mappings >====
impl_ezql_types!(
    i32 => Integer(),
    String => VarChar(255),
    bool => Boolean(),
);
