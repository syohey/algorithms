fn main() {
    // let mut arr = vec![3, 1, 2, -4, 5];
    // let target = 6;

    // // arr.sort();
    // // println!("{:?}", arr);
    // println!("{:?}", two_num_sum(&mut arr, target));
    // let mut arr1 = vec![2, 3, 4, 5, 6];
    // let mut arr2 = vec![3, 5];

    // println!("{}", validate_sequence(&mut arr1, &mut arr2));

    // let mut arr = vec![12, 3, 1, 2, -6, 5, -8, 6];
    // let target = 0;
    // println!("{:?}", three_num_sum(&mut arr, target));
    // let mut arr = vec![];
    // let a = &mut vec![1, 2, 3];
    // let b = &mut vec![4, 5, 6];
    // arr.push(a);
    // arr.push(b);
    // println!("{:?}", arr);
    // let mut arr1 = vec![-1, 5, 10, 20, 28, 3];
    // let mut arr2 = vec![26, 134, 135, 15, 17];
    // println!("{:?}", smalledst_difference(&mut arr1, &mut arr2));
    // let mut arr = vec![4, 1, 2, 2, 2, 3, 2, 2];
    // let target = 2;
    // println!("{:?}", move_elem_to_end(&mut arr, target));
    // let arr = vec![-1, -5, -10, -100, -100, -1010, -1011];
    // println!("{}", is_monotonic(arr));

    // let arr = vec![
    //     vec![1, 2, 3, 4],
    //     vec![12, 13, 14, 5],
    //     vec![11, 16, 15, 6],
    //     vec![10, 9, 8, 7],
    // ];
    // println!("{:?}", spiral_traverse(&arr));

    let arr = vec![1, 2, 3, 3, 4, 0, 10, 6, 5, -1, -3, 2, 3];
    println!("{}", longest_peak(&arr));
}

// TWO NUMBER SUM
/*
    Given an array of integers and a target value, return an array of two integers that sum up to the target value.
    Return an empty array otherwise.
*/
#[allow(dead_code)]
fn two_num_sum(arr: &mut Vec<i32>, target: i32) -> Vec<i32> {
    arr.sort();
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let current_sum = arr[left] + arr[right];

        if current_sum == target {
            return vec![arr[left], arr[right]];
        }

        if current_sum < target {
            left += 1;
        } else if current_sum > target {
            right -= 1;
        }
    }

    vec![]
}

// THREE NUMBER SUM
#[allow(dead_code)]
fn three_num_sum(arr: &mut Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut combinations = vec![];
    arr.sort();

    for i in 0..arr.len() - 2 {
        let mut left = i + 1;
        let mut right = arr.len() - 1;

        while left < right {
            let current_sum = arr[i] + arr[left] + arr[right];
            if current_sum == target {
                combinations.push(vec![arr[i], arr[left], arr[right]]);
                left += 1;
                right -= 1;
            }

            if current_sum < target {
                left += 1;
            } else if current_sum > target {
                right -= 1;
            }
        }
    }

    combinations
}

// VALIDATE SEQUENCE
/*
    Given two non-empty arrays, validate that the second array is a subsequence of the first.
*/
#[allow(dead_code)]
fn validate_sequence(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> bool {
    arr1.sort();
    arr2.sort();

    let mut first = 0;
    let mut second = 0;

    while first < arr1.len() && second < arr2.len() {
        if arr2[second] == arr1[first] {
            first += 1;
            second += 1;
        } else {
            first += 1;
        }
    }

    second == arr2.len()
}

// SMALLEST DIFFERENCE
#[allow(dead_code)]
fn smalledst_difference(arr1: &mut Vec<i32>, arr2: &mut Vec<i32>) -> Vec<i32> {
    arr1.sort();
    arr2.sort();
    let mut idx1 = 0;
    let mut idx2 = 0;
    let mut smallest = f64::INFINITY;
    // let mut current = f64::INFINITY;
    let mut smallest_pair = vec![];
    while idx1 < arr1.len() && idx2 < arr2.len() {
        let first = arr1[idx1];
        let second = arr2[idx2];
        if first == second {
            return vec![first, second];
        }
        let current = (first - second).abs().into();
        if current < smallest {
            smallest = current;
            smallest_pair = vec![first, second];
        }
        if first < second {
            idx1 += 1;
        } else if first > second {
            idx2 += 1;
        }
    }

    smallest_pair
}

// MOVE ELEMENT TO END
/*
    Given an array of integers and a target value, move the numbers that are the same as target to the end of the array.
*/
#[allow(dead_code)]
fn move_elem_to_end(arr: &mut Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        while left < right && arr[right] == target {
            right -= 1;
        }
        if arr[left] == target {
            let temp = arr[left];
            arr[left] = arr[right];
            arr[right] = temp;
        }
        left += 1;
    }
    arr.to_vec()
}

// MONOTONIC ARRAY
/*
    given an array of integers, determine if the array is monotonic.
*/
#[allow(dead_code)]
fn is_monotonic(arr: Vec<i32>) -> bool {
    if arr.len() <= 2 {
        return true;
    }

    let mut direction = arr[1] - arr[0];
    for i in 2..arr.len() {
        if direction == 0 {
            direction = arr[i] - arr[i - 1];
            continue;
        }

        if breaks_direction(direction, arr[i - 1], arr[i]) {
            return false;
        }
    }

    true
}

fn breaks_direction(direction: i32, prev: i32, curr: i32) -> bool {
    let diff = curr - prev;

    if direction > 0 {
        return diff < 0;
    } else {
        return diff > 0;
    }
}

// SPIRAL TRAVERSE
/*
    [[ 1,  2,  3, 4],
     [12, 13, 14, 5],  ==> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
     [11, 16, 15, 6],
     [10,  9,  8, 7]]
*/
#[allow(dead_code)]
fn spiral_traverse(arr: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let mut start_col = 0;
    let mut end_col = arr[0].len() - 1;
    let mut start_row = 0;
    let mut end_row = arr.len() - 1;

    while start_col <= end_col && start_row <= end_row {
        for col in start_col..end_col + 1 {
            result.push(arr[start_row][col]);
        }

        for row in start_row + 1..end_row + 1 {
            result.push(arr[row][end_col]);
        }

        for col in (start_col..end_col).rev() {
            if start_row == end_row {
                break;
            }
            result.push(arr[end_row][col]);
        }

        for row in (start_row + 1..end_row).rev() {
            if start_col == end_col {
                break;
            }
            result.push(arr[row][start_col]);
        }

        start_col += 1;
        end_col -= 1;
        start_row += 1;
        end_row -= 1;
    }

    result
}

// LONGEST PEAK
#[allow(dead_code)]
fn longest_peak(arr: &Vec<i32>) -> usize {
    let mut longest_peak_length = 0;
    let mut i = 1;

    while i < arr.len() - 1 {
        let is_peak = arr[i - 1] < arr[i] && arr[i] > arr[i + 1];

        if !is_peak {
            i += 1;
            continue;
        }

        let mut left_idx = i - 2;
        while arr[left_idx] < arr[left_idx + 1] {
            left_idx -= 1;
        }
        let mut right_idx = i + 2;
        while right_idx < arr.len() && arr[right_idx] < arr[right_idx - 1] {
            right_idx += 1;
        }

        let current_peak_length = right_idx - left_idx - 1;
        if longest_peak_length < current_peak_length {
            longest_peak_length = current_peak_length;
        }
        i = right_idx
    }

    longest_peak_length
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_num_sum() {
        assert_eq!(two_num_sum(&mut vec![2, 1, 3], 3), vec![1, 2]);
        assert_eq!(two_num_sum(&mut vec![3, 1, 2, -4, 5], 6), vec![1, 5]);
        assert_eq!(two_num_sum(&mut vec![0], 3), vec![]);
        assert_eq!(two_num_sum(&mut vec![3, 3, 3, 3, 2, 4], 5), vec![2, 3]);
        assert_eq!(two_num_sum(&mut vec![-5, 4, -1, 3], -6), vec![-5, -1]);
        assert_eq!(two_num_sum(&mut vec![100, 0], 99), vec![]);
    }

    #[test]
    fn test_validate_sequence() {
        assert_eq!(validate_sequence(&mut vec![1, 2, 3], &mut vec![2]), true);
        assert_eq!(
            validate_sequence(&mut vec![5, 4, 3, 6, 1], &mut vec![3, 4]),
            true
        );
        assert_eq!(
            validate_sequence(&mut vec![-100, 0, 4, 5], &mut vec![]),
            true
        );
    }
}
