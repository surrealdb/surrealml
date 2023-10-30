//! Defines a generic string value for the header.


/// Defines a generic string value for the header.
/// 
/// # Fields
/// * `value` - The value of the string.
#[derive(Debug, PartialEq)]
pub struct StringValue {
    pub value: Option<String>,
}


impl StringValue {

    /// Creates a new string value with no value.
    /// 
    /// # Returns
    /// A new string value with no value.
    pub fn fresh() -> Self {
        StringValue {
            value: None,
        }
    }

    /// Creates a new string value from a string.
    /// 
    /// # Arguments
    /// * `value` - The value of the string.
    /// 
    /// # Returns
    /// A new string value.
    pub fn from_string(value: String) -> Self {
        match value.as_str() {
            "" => StringValue {
                value: None,
            },
            _ => StringValue {
                value: Some(value),
            },
        }
    }

    /// Converts the string value to a string.
    /// 
    /// # Returns
    /// The string value as a string.
    pub fn to_string(&self) -> String {
        match &self.value {
            Some(value) => value.to_string(),
            None => String::from(""),
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh() {
        let string_value = StringValue::fresh();
        assert_eq!(string_value, StringValue {
            value: None,
        });
    }

    #[test]
    fn test_from_string() {
        let string_value = StringValue::from_string(String::from("test"));
        assert_eq!(string_value, StringValue {
            value: Some(String::from("test")),
        });
    }

    #[test]
    fn test_from_string_none() {
        let string_value = StringValue::from_string(String::from(""));
        assert_eq!(string_value, StringValue {
            value: None,
        });
    }

    #[test]
    fn test_to_string() {
        let string_value = StringValue::from_string(String::from("test"));
        assert_eq!(string_value.to_string(), String::from("test"));
    }

    #[test]
    fn test_to_string_none() {
        let string_value = StringValue {
            value: None,
        };
        assert_eq!(string_value.to_string(), String::from(""));
    }
}