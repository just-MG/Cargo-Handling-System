//Initial version 
// fn logic(red: i32, green:i32, blue:i32) -> String {
//     let (r, g, b): (u8, u8, u8) = (define_red(red), define_green(green), define_blue(blue));

//     match (r,g,b){
//         (1,1,1) => "White disc",
//         (2,2,2) => "Black disc",
//         (3,3,3) => "Conveyer",
//         _ => "Unknown"
//     }
// }

// fn define_red (red: i32) -> u8 {
//     //Red lower and upper boundries for white disk 
//     let (wd,wu): (i32, i32) = (50,60);
//     //Red lower and upper boundries for black disk
//     let (bd,bu): (i32, i32) = (135,144);
//     //Red lower and upper boundries for conver belt
//     let (cd,cu): (i32, i32) = (145,168);
    
//     match red {
//         // 1 - white, 2- black, 3-conveyer, 0-unknown
//         wd..=wu => 1,
//         bd..=bu => 2,
//         cd..=cu => 3,
//         _ => 0
//     }
// }
// fn define_green (green: i32) -> u8 {
//     //Green lower and upper boundries for white disk 
//     let (wd,wu): (i32, i32) = (60,80);
//     //Green lower and upper boundries for black disk
//     let (bd,bu): (i32, i32) = (165,185);
//     //Green lower and upper boundries for conver belt
//     let (cd,cu): (i32, i32) = (220,240);
    
//     match green {
//         // 1 - white, 2- black, 3-conveyer, 0-unknown
//         wd..=wu => 1,
//         bd..=bu => 2,
//         cd..=cu => 3,
//         _ => 0
//     }
// }
// fn define_blue (blue: i32) -> u8 {
//     //Blue lower and upper boundries for white disk 
//     let (wd,wu): (i32, i32) = (60,80);
//     //Blue lower and upper boundries for black disk
//     let (bd,bu): (i32, i32) = (165,185);
//     //Blue lower and upper boundries for conver belt
//     let (cd,cu): (i32, i32) = (220,240);
    
//     match blue {
//         // 1 - white, 2- black, 3-conveyer, 0-unknown
//         wd..=wu => 1,
//         bd..=bu => 2,
//         cd..=cu => 3,
//         _ => 0
//     }
// }

//Optimalized version
// Define a struct to hold the RGB color ranges for each classification
struct ColorRange {
    white: (i32, i32),
    black: (i32, i32),
    conveyor: (i32, i32),
}

impl ColorRange {
    // Constructor for creating a new ColorRange instance
    fn new(white: (i32, i32), black: (i32, i32), conveyor: (i32, i32)) -> Self {
        ColorRange { white, black, conveyor }
    }

    // Method to classify a given color value into one of the predefined categories
    fn classify(&self, value: i32) -> u8 {
        match value {
            _ if (self.white.0..=self.white.1).contains(&value) => 1, // White range
            _ if (self.black.0..=self.black.1).contains(&value) => 2, // Black range
            _ if (self.conveyor.0..=self.conveyor.1).contains(&value) => 3, // Conveyor range
            _ => 0, // Unknown category
        }
    }
}

// Main logic function to determine the classification based on RGB values
pub fn logic(red: i32, green: i32, blue: i32) -> String {
    // Initialize the ranges for each color
    let red_range = ColorRange::new((50, 60), (135, 144), (145, 168));
    let green_range = ColorRange::new((60, 80), (165, 185), (220, 240));
    let blue_range = ColorRange::new((60, 80), (165, 185), (220, 240));

    // Classify each color component
    let r = red_range.classify(red);
    let g = green_range.classify(green);
    let b = blue_range.classify(blue);

    // Match the classified results to determine the object type
    match (r, g, b) {
        (1, 1, 1) => "White disk".to_string(),  // All colors match white disk range
        (2, 2, 2) => "Black disk".to_string(),  // All colors match black disk range
        (3, 3, 3) => "Conveyor".to_string(),    // All colors match conveyor belt range
        _ => "Unknown".to_string(),             // Any other combination
    }
}


