//! Implements [selection algorithms](http://en.wikipedia.org/wiki/Selection_algorithm) on lists.

extern crate rand;

/// Given a list of N things, returns the K-th largest element.
///
/// Assumes 0 <= K < N.
pub trait KthLargest<T: Ord> {
    fn kth_largest<'a>(&self, list: &'a mut [T], k: usize) -> &'a T;
}

/// [Randomized selection](http://en.wikipedia.org/wiki/Quickselect) -- runs in O(n)
pub struct QuickSelect;

fn kth_largest_helper<'a, T: Ord>(list: &'a mut [T], mut left: usize, mut right: usize, k: usize) -> &'a T {
    if left == right { &list[left] }
    else {
        loop {
            let pivot = if right == left { left }
            else { left + rand::random::<usize>() % (right - left) };
            let pivot = partition(list, left, right, pivot);

            if k == pivot { return &list[k] }
            else if k < pivot {
                right = pivot - 1;
            } else {
                left = pivot + 1;
            }
        }
    }
}

fn partition<T: Ord>(list: &mut [T], left: usize, right: usize, pivot: usize) -> usize {
    list.swap(pivot, right);

    let mut store_index = left;
    for i in left..right {
        if list[i] < list[right] {
            list.swap(store_index, i);
            store_index += 1;
        }
    }

    list.swap(right, store_index);
    store_index
}

impl<T: Ord> KthLargest<T> for QuickSelect {
    fn kth_largest<'a>(&self, list: &'a mut [T], k: usize) -> &'a T {
        if k >= list.len() {
            panic!("tried to find {}-largest on {} elements, k is too big", k, list.len())
        } else {
            let right = list.len() - 1;
            kth_largest_helper(list, 0, right, k)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let list = vec![5usize, 1, 4, 3, 2, 0];
        for i in 0..list.len() {
            assert_eq!(*QuickSelect.kth_largest(&mut list.clone(), i), i);
        }
    }

    #[test]
    #[should_panic]
    fn zero_length_list() {
        let empty: &mut[isize] = &mut [];
        QuickSelect.kth_largest(empty, 0);
    }

    #[test]
    #[should_panic]
    fn k_too_big() {
        let mut list = vec![0usize, 1, 2];
        QuickSelect.kth_largest(&mut list, 3);
    }
}