use super::ezql_types::{EzqlType, EzqlValue};

// ====< Macro for generating type mappings >====
#[macro_export]
macro_rules! impl_ezql_types {
    ($($type:ty => $ezql_type:ident ($($argT:expr),*)),*,) => {
        $(
            impl From<$type> for EzqlType {
                fn from(_: $type) -> Self {
                    EzqlType::$ezql_type($($argT),*)
                }
            }

            impl From<$type> for EzqlValue {
                fn from(value: $type) -> Self {
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
    &str => VarChar(255),
    bool => Boolean(),
);
