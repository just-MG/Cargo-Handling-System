// Define a struct to hold the RGB color ranges for each classification
struct ColorRange {
    white: (i32, i32),
    black: (i32, i32),
    conveyor: (i32, i32),
}

impl ColorRange {
    /// Constructor for creating a new ColorRange instance
    fn new(white: (i32, i32), black: (i32, i32), conveyor: (i32, i32)) -> Self {
        ColorRange {
            white,
            black,
            conveyor,
        }
    }

    /// Method to classify a given color value into one of the predefined categories
    fn classify(&self, value: i32) -> u8 {
        match value {
            _ if (self.white.0..=self.white.1).contains(&value) => 1, // White range
            _ if (self.black.0..=self.black.1).contains(&value) => 2, // Black range
            _ if (self.conveyor.0..=self.conveyor.1).contains(&value) => 3, // Conveyor range
            _ => 0,                                                   // Unknown category
        }
    }
}

/// Main logic function to determine the classification based on RGB values. |
/// 0 - white disk |
/// 1 - black disk |
/// -1 - conveyor belt |
/// 2 - unknown object
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
