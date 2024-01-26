//! Utility functions for normalisers to reduce code duplication in areas that cannot be easily defined in a struct.
use regex::Regex;


/// Extracts the label from a normaliser string.
/// 
/// # Arguments
/// * `data` - A string containing the normaliser data.
pub fn extract_label(data: &String) -> String {
    let re = Regex::new(r"^(.*?)\(").unwrap();
    re.captures(data).unwrap().get(1).unwrap().as_str().to_string()
}


/// Extracts two numbers from a string with brackets where the numbers are in the brackets seperated by comma.
/// 
/// # Arguments
/// * `data` - A string containing the normaliser data.
/// 
/// # Returns
/// [number1, number2] from `"(number1, number2)"`
pub fn extract_two_numbers(data: &String) -> [f32; 2] {
    let re = Regex::new(r"[-+]?\d+(\.\d+)?").unwrap();
    let mut numbers = re.find_iter(data);
    let mut buffer: [f32; 2] = [0.0, 0.0];

    buffer[0] = numbers.next().unwrap().as_str().parse::<f32>().unwrap();
    buffer[1] = numbers.next().unwrap().as_str().parse::<f32>().unwrap();
    buffer
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_two_numbers() {
        let data = "linear_scaling(0.0,1.0)".to_string();
        let numbers = extract_two_numbers(&data);
        assert_eq!(numbers[0], 0.0);
        assert_eq!(numbers[1], 1.0);

        let data = "linear_scaling(0,1)".to_string();
        let numbers = extract_two_numbers(&data);
        assert_eq!(numbers[0], 0.0);
        assert_eq!(numbers[1], 1.0);
    }

    #[test]
    fn test_extract_label() {
        let data = "linear_scaling(0.0,1.0)".to_string();
        let label = extract_label(&data);
        assert_eq!(label, "linear_scaling");
    }
}