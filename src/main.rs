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
        let mut sorter = SortStruct::new(test_vector_1);
        sorter.bubble_sort();
        assert_eq!(sorter.vector, sorted_vector_1);
    }
}
