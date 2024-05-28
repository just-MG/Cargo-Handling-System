/*
TODO: change the doc below
Algorithm for sorting the discs into correct bins

Stores:
    - patters for the discs
Input: 
    - current state of the bins
    - output to be created
    - dick detected
Returs:
    - the bin where the disc should be placed
    - whether the disc is needed

White disk: 0
Black disk: 1
Other/unknown: 2 - the below code will not be called if disk is other or unknown
*/

/// Finds to what bin should a disk go
pub fn sort_disc(bins: &(Vec<i32>, Vec<i32>, Vec<i32>), output: [[u8; 5]; 3], disc: &i32) -> i32 {
    let next_bin1 = get_next_needed_bin(&bins.0, output, 0);
    let next_bin2 = get_next_needed_bin(&bins.1, output, 1);
    let next_bin3 = get_next_needed_bin(&bins.2, output, 2);

    // sort the disc
    // try the first bin
    if *disc == next_bin1 {
        return 0;
    } else {
        // try the second bin
        if *disc == next_bin2 {
            return 1
        } else {
            // try the third bin
            if *disc == next_bin3 {
                return 2;
            }
        }
    }
    // if error in sorting (should not happen)
    return -1
}

/// Checks whether the disk is needed in any bin
pub fn check_needed(bins: &(Vec<i32>, Vec<i32>, Vec<i32>), output: [[u8; 5]; 3], disc: &i32) -> bool {
    let next_bin1 = get_next_needed_bin(&bins.0, output, 0);
    let next_bin2 = get_next_needed_bin(&bins.1, output, 1);
    let next_bin3 = get_next_needed_bin(&bins.2, output, 2);
    return *disc == next_bin1 || *disc == next_bin2 || *disc == next_bin3;
}

fn get_next_needed_bin(bin: &Vec<i32>, output: [[u8; 5]; 3], bin_number: i32) -> i32 {
    // find the next disk needed for the bin
    if bin.len() == 5 {
        return -1; // bin is full
    } 
    return output[bin_number as usize][bin.len()].into();
}
