use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn get_json_entry<P: AsRef<Path>>(file_path: P, key: char) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    // Open the file in read-only mode with a buffer.
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Parse the JSON content.
    let json_value: Value = serde_json::from_reader(reader)?;

    // Convert the character to a string key.
    let key_str = key.to_string();

    // Access the entry at the specified key.
    if let Some(entry) = json_value.get(&key_str) {
        Ok(Some(entry.clone()))
    } else {
        Ok(None)
    }

    
}

pub fn get_predefined(index: char) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "predefined_output\\predefined_output.JSON";

    match get_json_entry(file_path, index)? {
        Some(entry) => {
            return Some(entry);
        }
        None => {
            println!("No entry found for key '{}'", key);
        }
    }

    Ok(())
}