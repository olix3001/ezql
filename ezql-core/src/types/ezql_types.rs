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

// ====< Pretty print EzqlType >====
impl std::fmt::Display for EzqlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EzqlType::Integer() => write!(f, "INTEGER"),
            EzqlType::VarChar(len) => write!(f, "VARCHAR({})", len),
            EzqlType::Boolean() => write!(f, "BOOLEAN"),
        }
    }
}
