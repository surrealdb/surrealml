//! Handles the loading, saving, and utilisation of all the data in the header of the model file.
pub mod keys;
pub mod normalisers;
pub mod output;

use keys::KeyBindings;
use normalisers::wrapper::NormaliserType;
use normalisers::NormaliserMap;
use output::Output;


/// The header of the model file.
/// 
/// # Fields
/// * `keys` - The key bindings where the order of the input columns is stored.
/// * `normalisers` - The normalisers where the normalisation functions are stored per column if there are any.
#[derive(Debug, PartialEq)]
pub struct Header {
    pub keys: KeyBindings,
    pub normalisers: NormaliserMap,
    pub output: Output,
}


impl Header {

    /// Creates a new header with no columns or normalisers.
    /// 
    /// # Returns
    /// A new header with no columns or normalisers.
    pub fn fresh() -> Self {
        Header {
            keys: KeyBindings::fresh(),
            normalisers: NormaliserMap::fresh(),
            output: Output::fresh(),
        }
    }

    /// Adds a column name to the `self.keys` field. It must be noted that the order in which the columns are added is 
    /// the order in which they will be expected in the input data. We can do this with the followng example:
    /// 
    /// ```rust
    /// use surrealml::storage::header::Header;
    /// 
    /// let mut header = Header::fresh();
    /// header.add_column("column_1".to_string());
    /// header.add_column("column_2".to_string());
    /// header.add_column("column_3".to_string());
    /// ```
    /// 
    /// # Arguments
    /// * `column_name` - The name of the column to be added.
    pub fn add_column(&mut self, column_name: String) {
        self.keys.add_column(column_name);
    }

    /// Adds a normaliser to the `self.normalisers` field.
    /// 
    /// # Arguments
    /// * `column_name` - The name of the column to which the normaliser will be applied.
    /// * `normaliser` - The normaliser to be applied to the column.
    pub fn add_normaliser(&mut self, column_name: String, normaliser: NormaliserType) {
        self.normalisers.add_normaliser(normaliser, column_name, &self.keys);
    }

    /// Adds an output column to the `self.output` field.
    /// 
    /// # Arguments
    /// * `column_name` - The name of the column to be added.
    /// * `normaliser` - The normaliser to be applied to the column.
    pub fn add_output(&mut self, column_name: String, normaliser: Option<NormaliserType>) {
        self.output.name = Some(column_name);
        self.output.normaliser = normaliser;
    }

    /// The standard delimiter used to seperate each field in the header.
    fn delimiter() -> &'static str {
        "//=>"
    } 

    /// Constructs the `Header` struct from bytes.
    /// 
    /// # Arguments
    /// * `data` - The bytes to be converted into a `Header` struct.
    /// 
    /// # Returns
    /// The `Header` struct.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, String> {
        let string_data = match String::from_utf8(data) {
            Ok(data) => data,
            Err(_) => return Err("Error converting bytes to string for header".to_string())
        };
        let buffer = string_data.split(Self::delimiter()).collect::<Vec<&str>>();
        let keys = KeyBindings::from_string(buffer[1].to_string())?;
        let normalisers = NormaliserMap::from_string(buffer[2].to_string(), &keys);
        let output = Output::from_string(buffer[3].to_string()); 
        Ok(Header {keys, normalisers, output})
    }

    /// Converts the `Header` struct into bytes.
    /// 
    /// # Returns
    /// A tuple containing the number of bytes in the header and the bytes themselves.
    pub fn to_bytes(&self) -> (i32, Vec<u8>) {
        let buffer = vec![
            "".to_string(),
            self.keys.to_string(),
            self.normalisers.to_string(),
            self.output.to_string(),
            "".to_string(),
        ];
        let buffer = buffer.join(Self::delimiter()).into_bytes();
        (buffer.len() as i32, buffer)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use super::keys::tests::generate_string as generate_key_string;
    use super::normalisers::tests::generate_string as generate_normaliser_string;
    use super::normalisers::{
        clipping::Clipping,
        linear_scaling::LinearScaling,
        log_scale::LogScaling,
        z_score::ZScore,
    };


    pub fn generate_string() -> String {
        let keys = generate_key_string();
        let normalisers = generate_normaliser_string();
        let output = "g=>linear_scaling(0.0,1.0)".to_string();
        format!(
            "{}{}{}{}{}{}{}", 
            Header::delimiter(), 
            keys, 
            Header::delimiter(), 
            normalisers, 
            Header::delimiter(),
            output,
            Header::delimiter(),
        )
    }

    pub fn generate_bytes() -> Vec<u8> {
        generate_string().into_bytes()
    }

    #[test]
    fn test_from_bytes() {
        let header = Header::from_bytes(generate_bytes()).unwrap();

        println!("{:?}", header);
        assert_eq!(header.keys.store.len(), 6);
        assert_eq!(header.keys.reference.len(), 6);
        assert_eq!(header.normalisers.store.len(), 4);

        assert_eq!(header.keys.store[0], "a");
        assert_eq!(header.keys.store[1], "b");
        assert_eq!(header.keys.store[2], "c");
        assert_eq!(header.keys.store[3], "d");
        assert_eq!(header.keys.store[4], "e");
        assert_eq!(header.keys.store[5], "f");
    }

    #[test]
    fn test_empty_header() {
        let string = "//=>//=>//=>//=>".to_string();
        let data = string.as_bytes();
        let header = Header::from_bytes(data.to_vec()).unwrap();
        assert_eq!(header, Header::fresh());
    }

    #[test]
    fn test_to_bytes() {
        let header = Header::from_bytes(generate_bytes()).unwrap();
        let (bytes_num, bytes) = header.to_bytes();
        let string = String::from_utf8(bytes).unwrap();

        // below the integers are correct but there is a difference with the decimal point representation in the string, we can alter this
        // fairly easy and will investigate it
        let expected_string = "//=>a=>b=>c=>d=>e=>f//=>a=>linear_scaling(0,1)//b=>clipping(0,1.5)//c=>log_scaling(10,0)//e=>z_score(0,1)//=>g=>linear_scaling(0,1)//=>".to_string();
        assert_eq!(string, expected_string);
        assert_eq!(bytes_num, expected_string.len() as i32);

        let empty_header = Header::fresh();
        let (bytes_num, bytes) = empty_header.to_bytes();
        let string = String::from_utf8(bytes).unwrap();
        let expected_string = "//=>//=>//=>//=>".to_string();

        assert_eq!(string, expected_string);
        assert_eq!(bytes_num, expected_string.len() as i32);
    }

    #[test]
    fn test_add_column() {
        let mut header = Header::fresh();
        header.add_column("a".to_string());
        header.add_column("b".to_string());
        header.add_column("c".to_string());
        header.add_column("d".to_string());
        header.add_column("e".to_string());
        header.add_column("f".to_string());

        assert_eq!(header.keys.store.len(), 6);
        assert_eq!(header.keys.reference.len(), 6);

        assert_eq!(header.keys.store[0], "a");
        assert_eq!(header.keys.store[1], "b");
        assert_eq!(header.keys.store[2], "c");
        assert_eq!(header.keys.store[3], "d");
        assert_eq!(header.keys.store[4], "e");
        assert_eq!(header.keys.store[5], "f");
    }

    #[test] 
    fn test_add_normalizer() {
        let mut header = Header::fresh();
        header.add_column("a".to_string());
        header.add_column("b".to_string());
        header.add_column("c".to_string());
        header.add_column("d".to_string());
        header.add_column("e".to_string());
        header.add_column("f".to_string());

        header.add_normaliser(
            "a".to_string(), 
            NormaliserType::LinearScaling(LinearScaling { min: 0.0, max: 1.0 })
        );
        header.add_normaliser(
            "b".to_string(), 
            NormaliserType::Clipping(Clipping { min: Some(0.0), max: Some(1.5) })
        );
        header.add_normaliser(
            "c".to_string(), 
            NormaliserType::LogScaling(LogScaling { base: 10.0, min: 0.0 })
        );
        header.add_normaliser(
            "e".to_string(), 
            NormaliserType::ZScore(ZScore { mean: 0.0, std_dev: 1.0 })
        );

        assert_eq!(header.normalisers.store.len(), 4);
        assert_eq!(header.normalisers.store[0], NormaliserType::LinearScaling(LinearScaling { min: 0.0, max: 1.0 }));
        assert_eq!(header.normalisers.store[1], NormaliserType::Clipping(Clipping { min: Some(0.0), max: Some(1.5) }));
        assert_eq!(header.normalisers.store[2], NormaliserType::LogScaling(LogScaling { base: 10.0, min: 0.0 }));
        assert_eq!(header.normalisers.store[3], NormaliserType::ZScore(ZScore { mean: 0.0, std_dev: 1.0 }));
    }

}


