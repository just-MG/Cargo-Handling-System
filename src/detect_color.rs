/// A struct to hold the RGB color ranges for each classification.
struct ColorRange {
    white: (i32, i32),
    black: (i32, i32),
    conveyor: (i32, i32),
}

impl ColorRange {
    /// Creates a new `ColorRange` instance.
    ///
    /// # Arguments
    ///
    /// * `white` - A tuple representing the min and max values for the white range.
    /// * `black` - A tuple representing the min and max values for the black range.
    /// * `conveyor` - A tuple representing the min and max values for the conveyor range.
    ///
    /// # Returns
    ///
    /// A new `ColorRange` instance with the specified ranges.
    fn new(white: (i32, i32), black: (i32, i32), conveyor: (i32, i32)) -> Self {
        ColorRange {
            white,
            black,
            conveyor,
        }
    }

    /// Classifies a given color value into one of the predefined categories.
    ///
    /// # Arguments
    ///
    /// * `value` - The color value to be classified.
    ///
    /// # Returns
    ///
    /// An integer representing the classification:
    /// * `1` for the white range.
    /// * `2` for the black range.
    /// * `3` for the conveyor range.
    /// * `0` for an unknown category.
    fn classify(&self, value: i32) -> u8 {
        match value {
            _ if (self.white.0..=self.white.1).contains(&value) => 1, // White range
            _ if (self.black.0..=self.black.1).contains(&value) => 2, // Black range
            _ if (self.conveyor.0..=self.conveyor.1).contains(&value) => 3, // Conveyor range
            _ => 0,                                                   // Unknown category
        }
    }
}

/// Determines the classification of an object based on RGB values.
///
/// # Arguments
///
/// * `color_values` - A tuple containing the RGB values as integers.
///
/// # Returns
///
/// An integer representing the object classification:
/// * `0` for a white disk.
/// * `1` for a black disk.
/// * `-1` for the conveyor belt.
/// * `2` for an unknown object.
pub fn logic(color_values: (i32, i32, i32)) -> i32 {
    // Initialize the ranges for each color
    let red_range = ColorRange::new((280, 400), (25, 110), (-15, 10));
    let green_range = ColorRange::new((280, 350), (30, 120), (-17, 5));
    let blue_range = ColorRange::new((270, 370), (18, 110), (-10, 10));

    // Classify each color component
    let r = red_range.classify(color_values.0);
    let g = green_range.classify(color_values.1);
    let b = blue_range.classify(color_values.2);

    // Match the classified results to determine the object type
    match (r, g, b) {
        (1, 1, 1) => 0,  // All colors match white disk range
        (2, 2, 2) => 1,  // All colors match black disk range
        (3, 3, 3) => -1, // All colors match conveyor belt range
        _ => 2,          // Any other combination
    }
}
