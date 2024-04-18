//! Defines the loading and saving functionality of normalisers.
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub mod traits;
pub mod utils;
pub mod linear_scaling;
pub mod clipping;
pub mod log_scale;
pub mod z_score;
pub mod wrapper;

use super::keys::KeyBindings;
use utils::{extract_label, extract_two_numbers};
use wrapper::NormaliserType;
use crate::safe_eject_option;
use crate::errors::error::{SurrealError, SurrealErrorStatus};


/// A map of normalisers so they can be accessed by column name and input index.
/// 
/// # Fields
/// * `store` - A vector of normalisers.
/// * `store_ref` - A vector of column names to correlate with the normalisers in the store.
/// * `reference` - A map of the index of the column in the key bindings to the index of the normaliser in the store.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NormaliserMap {
    pub store: Vec<NormaliserType>,
    pub store_ref: Vec<String>,
    pub reference: HashMap<usize, usize>,
}

impl NormaliserMap {

    /// Constructs a new, empty `NormaliserMap`.
    /// 
    /// # Returns
    /// A new, empty `NormaliserMap`.
    pub fn fresh() -> Self {
        NormaliserMap {
            store: Vec::new(),
            store_ref: Vec::new(),
            reference: HashMap::new(),
        }
    }

    /// Adds a normaliser to the map.
    /// 
    /// # Arguments
    /// * `normaliser` - The normaliser to add.
    /// * `column_name` - The name of the column to which the normaliser is applied.
    /// * `keys_reference` - A reference to the key bindings to extract the index.
    pub fn add_normaliser(&mut self, normaliser: NormaliserType, column_name: String, keys_reference: &KeyBindings) -> Result<(), SurrealError> {
        let counter = self.store.len();
        let column_input_index = safe_eject_option!(keys_reference.reference.get(column_name.as_str()));
        self.reference.insert(column_input_index.clone() as usize, counter as usize);
        self.store.push(normaliser);
        self.store_ref.push(column_name);
        Ok(())
    }

    /// Gets a normaliser from the map.
    /// 
    /// # Arguments
    /// * `column_name` - The name of the column to which the normaliser is applied.
    /// * `keys_reference` - A reference to the key bindings to extract the index.
    /// 
    /// # Returns
    /// The normaliser corresponding to the column name.
    pub fn get_normaliser(&self, column_name: String, keys_reference: &KeyBindings) -> Result<Option<&NormaliserType>, SurrealError> {
        let column_input_index = safe_eject_option!(keys_reference.reference.get(column_name.as_str()));
        let normaliser_index = self.reference.get(column_input_index);
        match normaliser_index {
            Some(normaliser_index) => Ok(Some(&self.store[*normaliser_index])),
            None => Ok(None),
        }
    }

    /// unpacks the normaliser data from a string.
    /// 
    /// # Arguments
    /// * `normaliser_data` - The string containing the normaliser data.
    /// 
    /// # Returns
    /// A tuple containing the label (type of normaliser), the numbers and the column name.
    pub fn unpack_normaliser_data(normaliser_data: &str) -> Result<(String, [f32; 2], String), SurrealError> {
        let mut normaliser_buffer = normaliser_data.split("=>");

        let column_name = safe_eject_option!(normaliser_buffer.next());
        let normaliser_type = safe_eject_option!(normaliser_buffer.next()).to_string();

        let label = extract_label(&normaliser_type)?;
        let numbers = extract_two_numbers(&normaliser_type)?;
        Ok((label, numbers, column_name.to_string()))
    }

    /// Constructs a `NormaliserMap` from a string.
    /// 
    /// # Arguments
    /// * `data` - The string containing the normaliser data.
    /// * `keys_reference` - A reference to the key bindings to extract the index.
    /// 
    /// # Returns
    /// A `NormaliserMap` containing the normalisers.
    pub fn from_string(data: String, keys_reference: &KeyBindings) -> Result<Self, SurrealError> {
        if data.len() == 0 {
            return Ok(NormaliserMap::fresh())
        }
        let normalisers_data = data.split("//");
        let mut counter = 0;
        let mut reference = HashMap::new();
        let mut store = Vec::new();
        let mut store_ref = Vec::new();

        for normaliser_data in normalisers_data {
            let (normaliser, column_name) = NormaliserType::from_string(normaliser_data.to_string())?;
            let column_input_index = safe_eject_option!(keys_reference.reference.get(column_name.as_str()));
            reference.insert(column_input_index.clone() as usize, counter as usize);
            store.push(normaliser);
            store_ref.push(column_name);
            counter += 1;
        }

        Ok(NormaliserMap {
            reference,
            store,
            store_ref
        })
    }

    /// Converts the `NormaliserMap` to a string.
    /// 
    /// # Returns
    /// A string containing the normaliser data.
    pub fn to_string(&self) -> String {
        let mut buffer = Vec::new();

        for index in 0..self.store.len() {
            let normaliser_string = &self.store[index].to_string();
            buffer.push(format!("{}=>{}", self.store_ref[index], normaliser_string));
        }

        buffer.join("//")
    }
}


#[cfg(test)]
pub mod tests {

    use super::*;
    use super::super::keys::tests::generate_string as generate_key_bindings_string;
    use super::super::keys::KeyBindings;

    pub fn generate_string() -> String {
        "a=>linear_scaling(0.0,1.0)//b=>clipping(0.0,1.5)//c=>log_scaling(10.0,0.0)//e=>z_score(0.0,1.0)".to_string()
    }

    pub fn generate_key_bindings() -> KeyBindings {
        let data = generate_key_bindings_string();
        KeyBindings::from_string(data)
    }

    #[test]
    pub fn test_from_string() {

        let key_bindings = generate_key_bindings();

        let data = generate_string();

        let normaliser_map = NormaliserMap::from_string(data, &key_bindings).unwrap();

        assert_eq!(normaliser_map.reference.len(), 4);
        assert_eq!(normaliser_map.store.len(), 4);

        assert_eq!(normaliser_map.reference.get(&0).unwrap(), &0);
        assert_eq!(normaliser_map.reference.get(&1).unwrap(), &1);
        assert_eq!(normaliser_map.reference.get(&2).unwrap(), &2);
        assert_eq!(normaliser_map.reference.get(&4).unwrap(), &3);
    }

    #[test]
    fn test_to_string() {     
        let key_bindings = generate_key_bindings();
        let data = generate_string();
        let normaliser_map = NormaliserMap::from_string(data, &key_bindings).unwrap();
        let normaliser_map_string = normaliser_map.to_string();

        assert_eq!(normaliser_map_string, "a=>linear_scaling(0,1)//b=>clipping(0,1.5)//c=>log_scaling(10,0)//e=>z_score(0,1)");
    }

    #[test]
    fn test_add_normalizer() {
            
            let key_bindings = generate_key_bindings();
            let data = generate_string();
    
            let mut normaliser_map = NormaliserMap::from_string(data, &key_bindings).unwrap();
    
            let _ = normaliser_map.add_normaliser(NormaliserType::LinearScaling(linear_scaling::LinearScaling{min: 0.0, max: 1.0}), "d".to_string(), &key_bindings);
    
            assert_eq!(normaliser_map.reference.len(), 5);
            assert_eq!(normaliser_map.store.len(), 5);
    
            assert_eq!(normaliser_map.reference.get(&0).unwrap(), &0);
            assert_eq!(normaliser_map.reference.get(&1).unwrap(), &1);
            assert_eq!(normaliser_map.reference.get(&2).unwrap(), &2);
            assert_eq!(normaliser_map.reference.get(&4).unwrap(), &3);
            assert_eq!(normaliser_map.reference.get(&3).unwrap(), &4);

            assert_eq!(normaliser_map.store_ref[0], "a");
            assert_eq!(normaliser_map.store_ref[1], "b");
            assert_eq!(normaliser_map.store_ref[2], "c");
            assert_eq!(normaliser_map.store_ref[3], "e");
            assert_eq!(normaliser_map.store_ref[4], "d");
    }

    #[test]
    fn test_get_normaliser() {
        let key_bindings = generate_key_bindings();
        let data = generate_string();

        let normaliser_map = NormaliserMap::from_string(data, &key_bindings).unwrap();

        let normaliser = normaliser_map.get_normaliser("e".to_string(), &key_bindings).unwrap().unwrap();

        match normaliser {
            NormaliserType::ZScore(z_score) => {
                assert_eq!(z_score.mean, 0.0);
                assert_eq!(z_score.std_dev, 1.0);
            },
            _ => panic!("Wrong normaliser type")
        }
    }
}

