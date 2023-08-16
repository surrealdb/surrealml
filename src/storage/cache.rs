//! Defines simple commands to load and save files in a cache folder whilst these files are being constructed.
//! This is to 
use crate::storage::surml_file::SurMlFile;
use uuid::Uuid;
use std::fs;


pub const CACHE_PATH: &str = "./.surmlcache";


pub fn create(file: SurMlFile) -> std::io::Result<String> {
    let uuid = Uuid::new_v4().to_string();
    let path = format!("{}/{}.surml", CACHE_PATH, uuid);
    establish_cache()?;
    file.write(&path)?;
    Ok(uuid)
}


pub fn save(file: SurMlFile, file_id: &String) -> std::io::Result<()> {
    let path = format!("{}/{}.surml", CACHE_PATH, file_id);
    establish_cache()?;
    file.write(&path)?;
    Ok(())
}


pub fn get(file_id: &String) -> std::io::Result<SurMlFile> {
    let path = format!("{}/{}.surml", CACHE_PATH, file_id);
    let file = SurMlFile::from_file(&path)?;
    Ok(file)
}


pub fn delete(file_id: &String) -> std::io::Result<()> {
    fs::remove_file(format!("{}/{}.surml", CACHE_PATH, file_id))?;
    Ok(())
}


fn establish_cache() -> std::io::Result<()> {
    match fs::metadata(&CACHE_PATH) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                fs::create_dir(CACHE_PATH)?;
            }
        },
        Err(_) => {
            fs::create_dir(CACHE_PATH)?;
        }
    }
    Ok(())
}


pub fn wipe_cache() -> std::io::Result<()> {
    if fs::metadata(CACHE_PATH)?.is_dir() {
        fs::remove_dir_all(CACHE_PATH)?;
    }
    Ok(())
}


/// Generates a unique identifier that can be used to access a loaded machine learning model.
/// 
/// # Returns
/// A unique identifier that can be used to access a loaded machine learning model.
pub fn generate_unique_id() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}
