use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Retrieves an entry from a JSON file based on a given key.
///
/// # Arguments
///
/// * `file_path` - A path to the JSON file.
/// * `key` - A character key to retrieve the entry.
///
/// # Returns
///
/// * An `Option` containing the JSON value associated with the key, if found.
/// * If the key is not found, returns `None`.
/// * Returns a `Result` indicating success or failure, with a boxed trait object for the error.
pub fn get_json_entry<P: AsRef<Path>>(file_path: P, key: char) -> Result<Option<Value>, Box<dyn std::error::Error>> {
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

/// Retrieves a predefined output array from a JSON file based on the provided index character.
///
/// # Arguments
///
/// * `index` - A character representing the index of the predefined output to retrieve.
///
/// # Returns
///
/// * A 2D array representing the predefined output.
///
/// # Panics
///
/// * Panics if the entry corresponding to the index is not found in the JSON file.
pub fn get_predefined(index: char) -> [[u8; 5]; 3] {
    let file_path = "predefined_output/predefined_output.JSON";
    let output = get_json_entry(file_path, index).unwrap();
    let output_array: [[u8; 5]; 3] = match output {
        Some(value) => serde_json::from_value(value).unwrap(),
        None => panic!("Entry not found in JSON"),
    };
    output_array
}
