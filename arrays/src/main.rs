fn main() {
    let mut arr = vec![3, 1, 2, -4, 5];
    let target = 6;

    // arr.sort();
    // println!("{:?}", arr);
    println!("{:?}", two_num_sum(&mut arr, target));
}

// two number sum
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
}
