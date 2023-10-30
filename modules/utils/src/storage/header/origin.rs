//! Defines the origin of the model in the file.
use super::string_value::StringValue;


const LOCAL: &str = "local";
const SURREAL_DB: &str = "surreal_db";
const NONE: &str = "";


/// Defines the types of origin that are supported.
/// 
/// # Fields
/// * `Local` - The model was created locally.
/// * `SurrealDb` - The model was created in the surreal database.
/// * `None` - The model has no origin
#[derive(Debug, PartialEq)]
pub enum OriginValue {
    Local(StringValue),
    SurrealDb(StringValue),
    None(StringValue),
}

impl OriginValue {

    /// Creates a new `OriginValue` with no value.
    /// 
    /// # Returns
    /// A new `OriginValue` with no value.
    pub fn fresh() -> Self {
        OriginValue::None(StringValue::fresh())
    }

    /// Create a `OriginValue` from a string.
    /// 
    /// # Arguments
    /// * `origin` - The origin as a string.
    /// 
    /// # Returns
    /// A new `OriginValue`.
    pub fn from_string(origin: String) -> Self {
        match origin.to_lowercase().as_str() {
            LOCAL => OriginValue::Local(StringValue::from_string(origin)),
            SURREAL_DB => OriginValue::SurrealDb(StringValue::from_string(origin)),
            NONE => OriginValue::None(StringValue::from_string(origin)),
            _ => panic!("Invalid origin."),
        }
    }

    /// Converts the `OriginValue` to a string.
    /// 
    /// # Returns
    /// The `OriginValue` as a string.
    pub fn to_string(&self) -> String {
        match self {
            OriginValue::Local(string_value) => string_value.to_string(),
            OriginValue::SurrealDb(string_value) => string_value.to_string(),
            OriginValue::None(string_value) => string_value.to_string(),
        }
    }

}


/// Defines the origin of the model in the file header.
/// 
/// # Fields
/// * `origin` - The origin of the model.
/// * `author` - The author of the model.
#[derive(Debug, PartialEq)]
pub struct Origin {
    pub origin: OriginValue,
    pub author: StringValue,
}


impl Origin {

    /// Creates a new origin with no values.
    /// 
    /// # Returns
    /// A new origin with no values.
    pub fn fresh() -> Self {
        Origin {
            origin: OriginValue::fresh(),
            author: StringValue::fresh(),
        }
    }

    /// Adds an author to the origin struct.
    /// 
    /// # Arguments
    /// * `origin` - The origin to be added.
    pub fn add_author(&mut self, author: String) {
        self.author = StringValue::from_string(author);
    }

    /// Adds an origin to the origin struct.
    /// 
    /// # Arguments
    pub fn add_origin(&mut self, origin: String) {
        self.origin = OriginValue::from_string(origin);
    }

    /// Converts an origin to a string.
    /// 
    /// # Returns
    /// The origin as a string.
    pub fn to_string(&self) -> String {
        if self.author.value.is_none() && self.origin == OriginValue::None(StringValue::fresh()) {
            return String::from("")
        }
        format!("{}=>{}", self.author.to_string(), self.origin.to_string())
    }

    /// Creates a new origin from a string.
    /// 
    /// # Arguments
    /// * `origin` - The origin as a string.
    /// 
    /// # Returns
    /// A new origin.
    pub fn from_string(origin: String) -> Self {
        if origin == "".to_string() {
            return Origin::fresh();
        }
        let mut split = origin.split("=>");
        let author = split.next().unwrap().to_string();
        let origin = split.next().unwrap().to_string();
        Origin {
            origin: OriginValue::from_string(origin),
            author: StringValue::from_string(author),
        }
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fresh() {
        let origin = Origin::fresh();
        assert_eq!(origin, Origin {
            origin: OriginValue::fresh(),
            author: StringValue::fresh(),
        });
    }

    #[test]
    fn test_to_string() {
        let origin = Origin {
            origin: OriginValue::from_string("local".to_string()),
            author: StringValue::from_string("author".to_string()),
        };
        assert_eq!(origin.to_string(), "author=>local".to_string());

        let origin = Origin::fresh();
        assert_eq!(origin.to_string(), "".to_string());
    }

    #[test]
    fn test_from_string() {
        let origin = Origin::from_string("author=>local".to_string());
        assert_eq!(origin, Origin {
            origin: OriginValue::from_string("local".to_string()),
            author: StringValue::from_string("author".to_string()),
        });

        let origin = Origin::from_string("=>local".to_string());

        assert_eq!(None, origin.author.value);
        assert_eq!("local".to_string(), origin.origin.to_string());
    }

}
