use super::*;

pub struct MergeSort;

// Split `slice` into two runs, sort both runs into `temp`, merge runs from `temp` into `slice`
fn split_merge<T: Clone + Ord + Debug>(in_slice: &mut [T], out_slice: &mut [T]) {
    // println!("SPLIT MERGE CALL");
    // println!("{:?}", in_slice);
    // println!("{:?}", out_slice);
    assert_eq!(in_slice.len(), out_slice.len());
    if in_slice.len() <= 1 {
        return;
    }

    let i_middle = in_slice.len() / 2;
    let (in_left, in_right) = in_slice.split_at_mut(i_middle);
    let (out_left, out_right) = out_slice.split_at_mut(i_middle);

    split_merge(out_left, in_left);
    split_merge(out_right, in_right);

    merge(in_left, in_right, out_left, out_right);
}

// precondition: out_left and out_right are sorted
// postcondition: [in_left, in_right].concat() is sorted
fn merge<T: Clone + Ord + Debug>(
    in_left: &mut [T],
    in_right: &mut [T],
    out_left: &mut [T],
    out_right: &mut [T],
) {
    // println!("MERGE CALL:");
    // println!("in left: {:?}", in_left);
    // println!("in right: {:?}", in_right);
    // println!("out left: {:?}", out_left);
    // println!("out right: {:?}", out_right);
    assert_eq!(in_left.len(), out_left.len());
    assert_eq!(out_right.len(), out_right.len());

    if (in_left.len() == 0) || (in_right.len() == 0) {
        return;
    }

    let mut i_insert = 0;
    let mut read_out_left = 0;
    let mut read_out_right = 0;

    while i_insert < in_left.len() + in_right.len() {
        if (read_out_left < out_left.len())
            && ((read_out_right == out_right.len())
                || (out_left[read_out_left] < out_right[read_out_right]))
        {
            if i_insert < in_left.len() {
                in_left[i_insert] = out_left[read_out_left].clone();
            } else {
                in_right[i_insert - in_left.len()] = out_left[read_out_left].clone();
            }
            read_out_left += 1;
        } else {
            if i_insert < in_left.len() {
                in_left[i_insert] = out_right[read_out_right].clone();
            } else {
                in_right[i_insert - in_left.len()] = out_right[read_out_right].clone();
            }
            read_out_right += 1;
        }
        i_insert += 1;
    }
}

impl Sorter for MergeSort {
    fn sort<T: Clone + Ord + Debug>(&self, slice: &mut [T]) {
        let mut temp: Vec<T> = Vec::new();
        temp.extend_from_slice(slice);
        split_merge(slice, &mut temp)
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    MergeSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5])
}
