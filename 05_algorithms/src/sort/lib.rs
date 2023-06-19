use std::fmt::Debug;

pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Clone + Debug;
}

mod bubblesort;
mod insertionsort;
mod mergesort;
mod quicksort;
mod selectionsort;
mod gnomesort; 

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort()
        }
    }
    #[test]
    fn std_works() {
        let mut things = vec![4, 3, 2, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
