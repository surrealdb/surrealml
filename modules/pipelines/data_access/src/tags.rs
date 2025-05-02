//! Defines a basic mechanism for reading tags from a file for a given video.
use std::fs::File;
use std::io::prelude::*;

use serde_json::Value;
use std::collections::HashMap;

/// Represents the different tags that can be assigned to a video frame.
///
/// # Variants
/// * `Preparation` - The first step of the surgery, where the patient is prepared for the surgery.
/// * `CarlotTriangleDissection` - The second step of the surgery, where the carlot triangle is dissected.
/// * `ClippingAndCutting` - Where clipping and cutting is happening in the video.
/// * `GallbladderDissection` - The fourth step of the surgery, where the gallbladder is dissected.
/// * `GallbladderPackaging` - The fifth step of the surgery, where the gallbladder is packaged.
/// * `CleaningAndCoagulation` - The sixth step of the surgery, where cleaning and coagulation is happening.
/// * `GallbladderExtraction` - The seventh step of the surgery, where the gallbladder is extracted.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SurgeryStep {
    Preparation,
    CarlotTriangleDissection,
    ClippingAndCutting,
    GallbladderDissection,
    GallbladderPackaging,
    CleaningAndCoagulation,
    GallbladderExtraction,
}

/// Converts a u8 to a SurgeryStep.
#[expect(clippy::fallible_impl_from)]
impl From<u8> for SurgeryStep {
    fn from(step: u8) -> Self {
        match step {
            0 => SurgeryStep::Preparation,
            1 => SurgeryStep::CarlotTriangleDissection,
            2 => SurgeryStep::ClippingAndCutting,
            3 => SurgeryStep::GallbladderDissection,
            4 => SurgeryStep::GallbladderPackaging,
            5 => SurgeryStep::CleaningAndCoagulation,
            6 => SurgeryStep::GallbladderExtraction,
            _ => panic!("Invalid step"),
        }
    }
}

impl SurgeryStep {
    /// Converts the surgery step to a u8.
    ///
    /// # Returns
    /// A u8 representing the surgery step.
    pub fn to_u8(&self) -> u8 {
        match self {
            SurgeryStep::Preparation => 0,
            SurgeryStep::CarlotTriangleDissection => 1,
            SurgeryStep::ClippingAndCutting => 2,
            SurgeryStep::GallbladderDissection => 3,
            SurgeryStep::GallbladderPackaging => 4,
            SurgeryStep::CleaningAndCoagulation => 5,
            SurgeryStep::GallbladderExtraction => 6,
        }
    }

    /// Converts the surgery step to a binary string of 4 bytes.
    ///
    /// # Returns
    /// A string containing the binary representation of the surgery step.
    pub fn to_binary_string(&self) -> String {
        format!("{:08b}", self.to_u8())
    }
}

/// Merely reads the tags from a file and returns them as a string.
///
/// # Note
/// Right now we are loading all of the tags into memory. This is not very efficient
/// if the size of the file grows but this will work for now to test things. We can
/// chunk the file later if we need to for loading and processing. I would recommend
/// that we come up with our own binary format to store the tags in so that we can read
/// chunks more efficiently and reduce the size of the file. as I think these files will
/// grow in size quite a bit.
///
/// # Arguments
/// * `path` - The path to the file containing the tags.
///
/// # Returns
/// A string containing the tags.
pub fn read_tags(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parses the tags in string format to a HashMap with surgical tags for indexes.
///
/// # Note
/// Right now we are merely loading the tags and putting them into a HashMap which is
/// not very efficient as we need to perform hashes per insert or lookup. Instead we
/// need to have a vector that has a pointer to the tags. The index can be the index
/// of the vector removing the need for hashing. However, for now this is fine just to
/// get things working. If we keep the interfaces the same, we can easily change the
/// indexing mechanism later without breaking the rest of the program.
///
/// # Arguments
/// * `data` - The string containing the tags.
///
/// # Returns
/// A HashMap with the tags for each index of the video.
pub fn parse_surgery_steps(data: String) -> HashMap<String, Vec<SurgeryStep>> {
    let v: Value = serde_json::from_str(&data).expect("Invalid JSON");

    let mut map = HashMap::new();

    for (key, steps) in v.as_object().expect("Expected a JSON object") {
        let steps_list = steps
            .as_array()
            .expect("Expected an array")
            .iter()
            .map(|step| SurgeryStep::from(step.as_u64().expect("Expected an integer") as u8))
            .collect();

        map.insert(key.to_string(), steps_list);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tags() {
        let tags = read_tags("./data_stash/cleaned_labels/VID68_processed.json").unwrap();
        let data = parse_surgery_steps(tags);
        assert_eq!(SurgeryStep::ClippingAndCutting, data.get("968").unwrap()[0]);
    }

    #[test]
    fn test_surgery_step_to_binary_string() {
        let step = SurgeryStep::GallbladderPackaging;
        assert_eq!("00000100", step.to_binary_string());
    }
}
