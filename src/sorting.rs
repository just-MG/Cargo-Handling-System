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

pub fn sort_disc(bins: &(Vec<i32>, Vec<i32>, Vec<i32>), pattern: &i32, disc: &i32) -> i32 {
    let (next_bin1, next_bin2, next_bin3) = get_next_needed(bins, pattern);

    // sort the disc
    // try the first bin
    if *disc == next_bin1 {
        return 1;
    } else {
        // try the second bin
        if *disc == next_bin2 {
            return 2;
        } else {
            // try the third bin
            if *disc == next_bin3 {
                return 3;
            }
        }
    }
    // if error in sorting (should not happen)
    return -1
}

pub fn check_needed(bins: &(Vec<i32>, Vec<i32>, Vec<i32>), pattern: &i32, disc: &i32) -> bool {
    let (next_bin1, next_bin2, next_bin3) = get_next_needed(bins, pattern);

    if !(*disc == next_bin1 || *disc == next_bin2 || *disc == next_bin3) {
        return false;
    }
    return true;
}

fn get_next_needed(bins: &(Vec<i32>, Vec<i32>, Vec<i32>), pattern: &i32) -> (i32, i32, i32) {
    let bin1 = &bins.0;
    let bin2 = &bins.1;
    let bin3 = &bins.2;

    // find the next disk needed for each bin
    // TODO: implement
    
    // Placeholder
    return (0, 1, 0)
}
