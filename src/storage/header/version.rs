//! Defines the process of managing the version of the `surml` file in the file.


/// The `Version` struct represents the version of the `surml` file.
/// 
/// # Fields
/// * `one` - The first number in the version.
/// * `two` - The second number in the version.
/// * `three` - The third number in the version.
#[derive(Debug, PartialEq)]
pub struct Version {
    pub one: u8,
    pub two: u8,
    pub three: u8,
}


impl Version {
    
    /// Creates a new `Version` struct with all zeros.
    /// 
    /// # Returns
    /// A new `Version` struct with all zeros.
    pub fn fresh() -> Self {
        Version {
            one: 0,
            two: 0,
            three: 0,
        }
    }

    /// Translates the struct to a string.
    /// 
    /// # Returns
    /// * `String` - The struct as a string.
    pub fn to_string(&self) -> String {
        if self.one == 0 && self.two == 0 && self.three == 0 {
            return "".to_string();
        }
        format!("{}.{}.{}", self.one, self.two, self.three)
    }

    /// Creates a new `Version` struct from a string.
    /// 
    /// # Arguments
    /// * `version` - The version as a string.
    /// 
    /// # Returns
    /// A new `Version` struct.
    pub fn from_string(version: String) -> Self {
        if version == "".to_string() {
            return Version::fresh();
        }
        let mut split = version.split(".");
        let one = split.next().unwrap().parse::<u8>().unwrap();
        let two = split.next().unwrap().parse::<u8>().unwrap();
        let three = split.next().unwrap().parse::<u8>().unwrap();
        Version {
            one,
            two,
            three,
        }
    }

    /// Increments the version by one.
    pub fn increment(&mut self) {
        self.three += 1;
        if self.three == 10 {
            self.three = 0;
            self.two += 1;
            if self.two == 10 {
                self.two = 0;
                self.one += 1;
            }
        }
    }
}


#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_from_string() {
        let version = Version::from_string("0.0.0".to_string());
        assert_eq!(version.one, 0);
        assert_eq!(version.two, 0);
        assert_eq!(version.three, 0);

        let version = Version::from_string("1.2.3".to_string());
        assert_eq!(version.one, 1);
        assert_eq!(version.two, 2);
        assert_eq!(version.three, 3);
    }

    #[test]
    fn test_to_string() {
        let version = Version{
            one: 0,
            two: 0,
            three: 0,
        };
        assert_eq!(version.to_string(), "");

        let version = Version{
            one: 1,
            two: 2,
            three: 3,
        };
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_increment() {
        let mut version = Version{
            one: 0,
            two: 0,
            three: 0,
        };
        version.increment();
        assert_eq!(version.to_string(), "0.0.1");

        let mut version = Version{
            one: 0,
            two: 0,
            three: 9,
        };
        version.increment();
        assert_eq!(version.to_string(), "0.1.0");

        let mut version = Version{
            one: 0,
            two: 9,
            three: 9,
        };
        version.increment();
        assert_eq!(version.to_string(), "1.0.0");

        let mut version = Version{
            one: 9,
            two: 9,
            three: 9,
        };
        version.increment();
        assert_eq!(version.to_string(), "10.0.0");
    }

}