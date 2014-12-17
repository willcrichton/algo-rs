pub trait KthLargest<T: Ord> {
    fn kth_largest<'a>(&self, list: &'a mut [T], k: uint) -> &'a T;
}

pub struct QuickSelect;

impl<T: Ord> QuickSelect {
    fn kth_largest_helper<'a>(&self, list: &'a mut [T], mut left: uint, mut right: uint, k: uint) -> &'a T {
        if left == right { &list[left] }
        else {
            loop {
                let pivot = (right + left) / 2;
                let pivot = self.partition(list, left, right, pivot);

                if k == pivot { return &list[k] }
                else if k < pivot {
                    right = pivot - 1;
                } else {
                    left = pivot + 1;
                }
            }
        }
    }

    fn partition(&self, list: &mut [T], left: uint, right: uint, pivot: uint) -> uint {
        list.swap(pivot, right);

        let mut store_index = left;
        for i in range(left, right) {
            if list[i] < list[right] {
                list.swap(store_index, i);
                store_index += 1;
            }
        }

        list.swap(right, store_index);
        store_index
    }
}

impl<T: Ord> KthLargest<T> for QuickSelect {
    fn kth_largest<'a>(&self, list: &'a mut [T], k: uint) -> &'a T {
        if k >= list.len() {
            panic!("tried to find {}-largest on {} elements, k is too big", k, list.len())
        } else {
            let right = list.len() - 1;
            self.kth_largest_helper(list, 0, right, k)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let list = vec![5u, 1, 4, 3, 2, 0];
        for i in range(0, list.len()) {
            assert_eq!(*QuickSelect.kth_largest(list.clone().as_mut_slice(), i), i);
        }
    }

    #[test]
    #[should_fail]
    fn zero_length_list() {
        let empty: &mut[int] = &mut [];
        QuickSelect.kth_largest(empty, 0);
    }

    #[test]
    #[should_fail]
    fn k_too_big() {
        let mut list = vec![0u, 1, 2];
        QuickSelect.kth_largest(list.as_mut_slice(), 3);
    }
}