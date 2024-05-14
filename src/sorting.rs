/*
Algorithm for sorting the discs into correct bins

Stores:
    - patters for the discs
Input: 
    - current state of the bins
    - pattern to be created
    - dick detected
Returs:
    - the bin where the disc should be placed
    - whether the disc is needed

White disk: 0
Black disk: 1
Other/unknown: 2 - the below code will not be called if disk is other or unknown
*/

pub fn sort_discs(bins: (Vec<i32>, Vec<i32>, Vec<i32>), pattern: i32, disc: i32) -> (i32, bool) {
    let bin1 = bins.0;
    let bin2 = bins.1;
    let bin3 = bins.2;
    let mut needed = false;

    // find the next disk needed for each bin
    // TODO: Get the next disk needed for each bin from the chosen data structure
    // set 2 if no more needed (bin full)
    // Placeholder
    let next_bin1 = 0;
    let next_bin2 = 0;
    let next_bin3 = 0;

    // check if the disk is needed
    if !(disc == next_bin1 || disc == next_bin2 || disc == next_bin3) {
        return (0, false);
    }
    
    // sort the disc
    // try the first bin
    if disc == next_bin1 {
        return (1, true)
    } else {
        // try the second bin
        if disc == next_bin2 {
            return (2, true)
        } else {
            // try the third bin
            if disc == next_bin3 {
                return (3, true)
            }
        }
    }
    // if error in sorting (should not happen)
    (-1, false)
}