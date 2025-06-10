fn partition(array: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = array[low];
    let (mut left, mut right) = (low, high);

    while left < right {
        while left < right && array[right] >= pivot {
            right -= 1;
        }
        array[left] = array[right];

        while left < right && array[left] <= pivot {
            left += 1;
        }
        array[right] = array[left];
    }

    array[left] = pivot;
    left
}

pub fn quick(array: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_loc = partition(array, low, high);
        quick(array, low, pivot_loc.saturating_sub(1));
        quick(array, pivot_loc + 1, high);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick() {
        let mut array = vec![5, 4, 3, 2, 1];
        let len = array.len();
        quick(&mut array, 0, len - 1);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }
}