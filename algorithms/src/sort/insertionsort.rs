use super::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // Divide the slice into sorted and unsorted
        // If self.smart = true, we use binary search
        // to find the insertion index. Note that this doesn't improve time complexity,
        // since we still have to do worst case n^2 shifts.
        for unsorted in 1..slice.len() {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) | Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1)
            }
        }
    }
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5])
}

#[test]
fn it_works_dumb() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5])
}
