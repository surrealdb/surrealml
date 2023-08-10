//! Defines the constructing and storing of normalisers.
use super::linear_scaling;
use super::clipping;
use super::log_scale;
use super::z_score;
use super::utils::{extract_label, extract_two_numbers};


/// A wrapper for all different types of normalisers.
/// 
/// # Arguments
/// * `LinearScaling` - A linear scaling normaliser.
/// * `Clipping` - A clipping normaliser.
/// * `LogScaling` - A log scaling normaliser.
/// * `ZScore` - A z-score normaliser.
#[derive(Debug, PartialEq)]
pub enum NormaliserType {
    LinearScaling(linear_scaling::LinearScaling),
    Clipping(clipping::Clipping),
    LogScaling(log_scale::LogScaling),
    ZScore(z_score::ZScore),
}


impl NormaliserType {

    /// Unpacks a normaliser from a string.
    /// 
    /// # Arguments
    /// * `normaliser_data` - A string containing the normaliser data.
    /// 
    /// # Returns
    /// (type of normaliser, [normaliser parameters], column name)
    pub fn unpack_normaliser_data(normaliser_data: &str) -> (String, [f32; 2], String) {
        let mut normaliser_buffer = normaliser_data.split("=>");
        let column_name = normaliser_buffer.next().unwrap();
        let normaliser_type = normaliser_buffer.next().unwrap().to_string();
        let label = extract_label(&normaliser_type);
        let numbers = extract_two_numbers(&normaliser_type);
        (label, numbers, column_name.to_string())
    }

    /// Constructs a normaliser from a string.
    /// 
    /// # Arguments
    /// * `data` - A string containing the normaliser data.
    /// 
    /// # Returns
    /// (normaliser, column name)
    pub fn from_string(data: String) -> Result<(Self, String), String> {
        let (label, numbers, column_name) = Self::unpack_normaliser_data(&data);
        let normaliser = match label.as_str() {
            "linear_scaling" => {
                let min = numbers[0];
                let max = numbers[1];
                NormaliserType::LinearScaling(linear_scaling::LinearScaling{min, max})
            },
            "clipping" => {
                let min = numbers[0];
                let max = numbers[1];
                NormaliserType::Clipping(clipping::Clipping{min: Some(min), max: Some(max)})
            },
            "log_scaling" => {
                let base = numbers[0];
                let min = numbers[1];
                NormaliserType::LogScaling(log_scale::LogScaling{base, min})
            },
            "z_score" => {
                let mean = numbers[0];
                let std_dev = numbers[1];
                NormaliserType::ZScore(z_score::ZScore{mean, std_dev})
            },
            _ => return Err(format!("Unknown normaliser type: {}", label))
        };
        Ok((normaliser, column_name))
    }

    /// Converts a normaliser to a string.
    /// 
    /// # Returns
    /// A string containing the normaliser data.
    pub fn to_string(&self) -> String {
        let normaliser_string = match self {
            NormaliserType::LinearScaling(linear_scaling) => {
                let min = linear_scaling.min;
                let max = linear_scaling.max;
                format!("linear_scaling({},{})", min, max)
            },
            NormaliserType::Clipping(clipping) => {
                let min = clipping.min.unwrap();
                let max = clipping.max.unwrap();
                format!("clipping({},{})", min, max)
            },
            NormaliserType::LogScaling(log_scaling) => {
                let base = log_scaling.base;
                let min = log_scaling.min;
                format!("log_scaling({},{})", base, min)
            },
            NormaliserType::ZScore(z_score) => {
                let mean = z_score.mean;
                let std_dev = z_score.std_dev;
                format!("z_score({},{})", mean, std_dev)
            },
        };
        normaliser_string
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    pub fn generate_string() -> String {
        let normaliser = NormaliserType::LinearScaling(linear_scaling::LinearScaling{min: 0.0, max: 1.0});
        let column_name = "column_name".to_string();
        format!("{}=>{}", column_name, normaliser.to_string())
    }

    #[test]
    fn test_normaliser_type_to_string() {
        let normaliser = NormaliserType::LinearScaling(linear_scaling::LinearScaling{min: 0.0, max: 1.0});
        assert_eq!(normaliser.to_string(), "linear_scaling(0,1)");
    }

    #[test]
    fn test_normaliser_type_from_string() {
        let normaliser_string = generate_string();
        let (normaliser, column_name) = NormaliserType::from_string(normaliser_string).unwrap();
        assert_eq!(normaliser, NormaliserType::LinearScaling(linear_scaling::LinearScaling{min: 0.0, max: 1.0}));
        assert_eq!(column_name, "column_name");
    }

}