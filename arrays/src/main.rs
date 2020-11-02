fn main() {
    // let mut arr = vec![3, 1, 2, -4, 5];
    // let target = 6;

    // // arr.sort();
    // // println!("{:?}", arr);
    // println!("{:?}", two_num_sum(&mut arr, target));
    // let mut arr1 = vec![2, 3, 4, 5, 6];
    // let mut arr2 = vec![3, 5];

    // println!("{}", validate_sequence(&mut arr1, &mut arr2));

    let mut arr = vec![12, 3, 1, 2, -6, 5, -8, 6];
    let target = 0;
    println!("{:?}", three_num_sum(&mut arr, target));
    // let mut arr = vec![];
    // let a = &mut vec![1, 2, 3];
    // let b = &mut vec![4, 5, 6];
    // arr.push(a);
    // arr.push(b);
    // println!("{:?}", arr);
}

// TWO NUMBER SUM
/*
    Given an array of integers and a target value, return an array of two integers that sum up to the target value.
    Return an empty array otherwise.
*/
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
