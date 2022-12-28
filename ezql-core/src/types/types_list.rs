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

// ====< Macro for generating inverse type mappings >====
#[macro_export]
macro_rules! impl_ezql_types_inverse {
    ($($type:ty => ($($ezql_type:ident),*)),*,) => {
        $(
            #[allow(clippy::from_over_into)]
            impl Into<$type> for EzqlValue {
                fn into(self) -> $type {
                    match self {
                        $(
                        EzqlValue::$ezql_type(v) => v.into()
                        ),*,
                        _ => panic!("Cannot convert {:?} to {}", self, stringify!($type)),
                    }
                }
            }

            #[allow(clippy::from_over_into)]
            impl Into<$type> for &EzqlValue {
                fn into(self) -> $type {
                    match self {
                        $(
                            EzqlValue::$ezql_type(v) => (*v).clone().into()
                        ),*,
                        _ => panic!("Cannot convert {:?} to {}", self, stringify!($type)),
                    }
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

// ====< Ezql type mappings >====
impl_ezql_types_inverse!(
    i32 => (Integer),
    String => (VarChar),
);

// ====< Custom type mappings >====
#[allow(clippy::from_over_into)]
impl Into<bool> for &EzqlValue {
    fn into(self) -> bool {
        match self {
            EzqlValue::Boolean(v) => *v,
            EzqlValue::Integer(v) => *v != 0,
            _ => panic!("Cannot convert {:?} to bool", self),
        }
    }
}
