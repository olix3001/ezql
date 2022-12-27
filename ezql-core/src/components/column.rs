use crate::{prelude::EzqlValue, types::EzqlType};

// ====< SQL column >====
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: EzqlType,
    pub properties: Vec<ColumnProperty>,
}

// ====< SQL column property >====
#[derive(Debug, Clone)]
pub enum ColumnProperty {
    PrimaryKey,
    NotNull,
    Unique,
    Default(EzqlValue),
}

// create default column property
impl ColumnProperty {
    pub fn default<T>(value: T) -> Self
    where
        T: Into<EzqlValue>,
    {
        ColumnProperty::Default(value.into())
    }
}

// ====< Impl column >====
impl Column {
    pub fn has_default(&self) -> bool {
        self.properties
            .iter()
            .any(|p| matches!(p, ColumnProperty::Default(_)))
    }

    pub fn get_default(&self) -> Option<EzqlValue> {
        self.properties.iter().find_map(|p| match p {
            ColumnProperty::Default(v) => Some(v.clone()),
            _ => None,
        })
    }

    pub fn is_primary_key(&self) -> bool {
        self.properties
            .iter()
            .any(|p| matches!(p, ColumnProperty::PrimaryKey))
    }
}

// ====< Pretty print column name >====
impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
