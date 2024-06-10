/// Finds to what bin number the currently detected disk should go to.
/// Based on the current state of the bins and the output.
/// Function is only called when the disk is determined to be needed.
///
/// # Arguments
/// * `bins` - A reference to an array of vectors representing the current state of the bins.
/// * `output` - A 2D array representing the required disk colors for each bin.
/// * `disc` - A reference to the currently detected disk.
///
/// # Returns
/// * `i32` - The bin number to which the disk should be sorted (0, 1, or 2), or -1 if the disk is not needed in any bin.
pub fn sort_disc(bins: &[Vec<i32>; 3], output: [[u8; 5]; 3], disc: &i32) -> i32 {
    let (next_bin1, next_bin2, next_bin3) = next_bins(bins, output);

    // sort the disc
    // try the first bin
    if *disc == next_bin1 {
        return 0;
    } else {
        // try the second bin
        if *disc == next_bin2 {
            return 1;
        } else {
            // try the third bin
            if *disc == next_bin3 {
                return 2;
            }
        }
    }
    // if function has been called incorrectly (disk is not needed in any bin)
    return -1;
}

/// Checks whether the disk is needed in any bin.
///
/// # Arguments
/// * `bins` - A reference to an array of vectors representing the current state of the bins.
/// * `output` - A 2D array representing the required disk colors for each bin.
/// * `disc` - A reference to the currently detected disk.
///
/// # Returns
/// * `bool` - Returns true if the disk is needed in any bin, otherwise false.
pub fn check_needed(bins: &[Vec<i32>; 3], output: [[u8; 5]; 3], disc: &i32) -> bool {
    let (next_bin1, next_bin2, next_bin3) = next_bins(bins, output);
    return *disc == next_bin1 || *disc == next_bin2 || *disc == next_bin3;
}

/// Gets the next needed disk (color) for each bin.
///
/// # Arguments
/// * `bins` - A reference to an array of vectors representing the current state of the bins.
/// * `output` - A 2D array representing the required disk colors for each bin.
///
/// # Returns
/// * `(i32, i32, i32)` - A tuple containing the next needed disk color for each bin.
fn next_bins(bins: &[Vec<i32>; 3], output: [[u8; 5]; 3]) -> (i32, i32, i32) {
    let next_bin1 = get_next_needed_bin(&bins[0], output, 0);
    let next_bin2 = get_next_needed_bin(&bins[1], output, 1);
    let next_bin3 = get_next_needed_bin(&bins[2], output, 2);
    return (next_bin1, next_bin2, next_bin3);
}

/// Finds the next disk color needed for a provided bin.
/// Returns -1 if the bin is full.
///
/// # Arguments
/// * `bin` - A reference to a vector representing the current state of a specific bin.
/// * `output` - A 2D array representing the required disk colors for each bin.
/// * `bin_number` - An integer representing the bin number.
///
/// # Returns
/// * `i32` - The next needed disk color for the specified bin, or -1 if the bin is full.
fn get_next_needed_bin(bin: &Vec<i32>, output: [[u8; 5]; 3], bin_number: i32) -> i32 {
    if bin.len() == 5 {
        return -1; // bin is full
    }
    return output[bin_number as usize][bin.len()].into();
}
