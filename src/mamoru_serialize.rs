use crate::component::guest::types::{ValueData, ValueType};
use std::collections::HashMap;

/// A builder for dynamically constructing data entries.
pub struct DataBuilder {
    /// A hashmap to hold the data entries with their names and corresponding values.
    inner_data: HashMap<&'static str, ValueData>,
}

impl DataBuilder {
    /// Adds an integer value to the data collection.
    ///
    /// Parameters:
    /// - `name`: The name of the data entry.
    /// - `value`: The integer value to store.
    ///
    /// Returns:
    /// A mutable reference to the builder to allow for chaining.
    pub fn integer(&mut self, name: &'static str, value: u64) -> &mut Self {
        self.inner_data.insert(
            name,
            ValueData {
                data: None,
                value: ValueType::U64(value),
            },
        );
        self
    }

    /// Adds a text string to the data collection.
    ///
    /// Parameters:
    /// - `name`: The name of the data entry.
    /// - `value`: The string value to store.
    ///
    /// Returns:
    /// A mutable reference to the builder to allow for chaining.
    pub fn text(&mut self, name: &'static str, value: String) -> &mut Self {
        self.inner_data.insert(
            name,
            ValueData {
                data: None,
                value: ValueType::String(value),
            },
        );
        self
    }

    /// Adds a boolean value to the data collection.
    ///
    /// Parameters:
    /// - `name`: The name of the data entry.
    /// - `value`: The boolean value to store.
    ///
    /// Returns:
    /// A mutable reference to the builder to allow for chaining.
    pub fn bool(&mut self, name: &'static str, value: bool) -> &mut Self {
        self.inner_data.insert(
            name,
            ValueData {
                data: None,
                value: ValueType::Bool(value),
            },
        );
        self
    }
}

/// Converts a `DataBuilder` instance into a `ValueData` structure.
///
/// This conversion allows the `DataBuilder` to be used directly where `ValueData` is required,
/// packaging the entire set of entries into a single `ValueData` item.
impl From<&DataBuilder> for ValueData {
    fn from(builder: &DataBuilder) -> Self {
        let mut data: Vec<ValueType> = Vec::new();
        let mut elems: Vec<(String, u8)> = Vec::new();

        for (index, (key, val)) in builder.inner_data.iter().enumerate() {
            //TODO it is not supported not primitive types
            if val.data.is_some() {
                continue;
            }
            data.push(val.value.clone());
            elems.push((key.to_string(), index as u8));
        }

        ValueData {
            data: Some(data),
            value: ValueType::Map(elems),
        }
    }
}

/// Converts the `DataBuilder` to `ValueData` using the implementation from the `From` trait.
///
/// Parameters:
/// - `data`: A reference to the `DataBuilder` instance.
///
/// Returns:
/// A `ValueData` instance constructed from the builder.
pub fn data(data: &DataBuilder) -> ValueData {
    data.into()
}
