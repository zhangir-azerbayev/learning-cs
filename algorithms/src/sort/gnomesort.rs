use super::Sorter;
use std::cmp::max; 

pub struct GnomeSort; 

impl Sorter for GnomeSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        if slice.len() <= 1 {return;}

        let mut i = 1; 
        while i < slice.len() {
            if slice[i-1] < slice[i] {
                i += 1; 
            } else {
                slice.swap(i-1, i); 
                i = max(1, i-1); 
            }

        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    GnomeSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5])
}
