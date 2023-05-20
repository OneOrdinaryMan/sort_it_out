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
        todo!()
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
        sorter_1.selection_sort();
        sorter_2.selection_sort();
        assert_eq!(sorter_1.vector, sorted_vector_1);
        assert_eq!(sorter_2.vector, sorted_vector_2);
    }
}
