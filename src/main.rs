struct SortStruct<T: Ord + Copy> {
    vector: Vec<T>,
}
impl<T: Ord + Copy> SortStruct<T> {
    fn new(input: Vec<T>) -> SortStruct<T> {
        SortStruct { vector: input }
    }
    fn bubble_sort(&mut self) {
        for i in 0..self.vector.len() - 1 {
            for j in 0..self.vector.len() - i - 1 {
                if self.vector[j] > self.vector[j + 1] {
                    self.vector.swap(j, j + 1);
                }
            }
        }
    }
    fn selection_sort(&mut self) {
        for i in 0..self.vector.len() - 1 {
            let mut min = i;
            for j in i + 1..self.vector.len() {
                if self.vector[min] > self.vector[j] {
                    min = j;
                }
            }
            self.vector.swap(i, min);
        }
    }
    fn insertion_sort(&mut self) {
        for i in 1..self.vector.len() {
            let mut j = i;
            while j > 0 && self.vector[j] < self.vector[j - 1] {
                self.vector.swap(j, j - 1);
                j -= 1;
            }
        }
    }
    fn merge_sort(&mut self) {
        SortStruct::merge_sort_recurssion(&mut self.vector);
    }
    fn merge_sort_recurssion(input: &mut Vec<T>) {
        if input.len() <= 1 {
            return;
        } else {
            let middle = input.len() / 2;
            let mut left_slice = Vec::new();
            let mut right_slice = Vec::new();
            for i in 0..input.len() {
                if i < middle {
                    left_slice.push(input[i]);
                } else {
                    right_slice.push(input[i]);
                }
            }
            SortStruct::merge_sort_recurssion(&mut left_slice);
            SortStruct::merge_sort_recurssion(&mut right_slice);
            SortStruct::merge(&mut left_slice, &mut right_slice, input);
        }
    }
    fn merge(left_slice: &mut Vec<T>, right_slice: &mut Vec<T>, input: &mut Vec<T>) {
        let l = input.len() / 2;
        let r = input.len() - l;
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < l && j < r {
            if left_slice[i] <= right_slice[j] {
                input[k] = left_slice[i];
                i += 1;
                k += 1;
            } else {
                input[k] = right_slice[j];
                j += 1;
                k += 1;
            }
        }
        while i < l {
            input[k] = left_slice[i];
            i += 1;
            k += 1;
        }
        while j < r {
            input[k] = right_slice[j];
            j += 1;
            k += 1;
        }
    }
    fn quick_sort(&mut self) {
        let len = self.vector.len() - 1;
        SortStruct::quick_sort_recursion(&mut self.vector, 0, len as isize);
    }
    fn quick_sort_recursion(input: &mut Vec<T>, start: isize, end: isize) {
        if end <= start {
            return;
        }
        let pivot = SortStruct::partition(input, start, end);
        SortStruct::quick_sort_recursion(input, start, pivot - 1);
        SortStruct::quick_sort_recursion(input, pivot + 1, end);
    }
    fn partition(input: &mut Vec<T>, start: isize, end: isize) -> isize {
        let pivot = input[end as usize];
        let mut i = start - 1;
        for j in start..end {
            if input[j as usize] < pivot {
                i += 1;
                input.swap(i as usize, j as usize);
            }
        }
        i += 1;
        input.swap(i as usize, end as usize);
        i
    }
}
fn main() {}
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn check_sorted() {
        let test_vector_1 = vec!["Hello Mother", "Hi Sis", "Hello Father", "Hi Bro"];
        let sorted_vector_1 = vec!["Hello Father", "Hello Mother", "Hi Bro", "Hi Sis"];
        let test_vector_2 = vec![10, 2, 3, 1];
        let sorted_vector_2 = vec![1, 2, 3, 10];
        let mut sorter_1 = SortStruct::new(test_vector_1);
        let mut sorter_2 = SortStruct::new(test_vector_2);
        sorter_1.quick_sort();
        sorter_2.quick_sort();
        assert_eq!(sorter_1.vector, sorted_vector_1);
        assert_eq!(sorter_2.vector, sorted_vector_2);
    }
}
