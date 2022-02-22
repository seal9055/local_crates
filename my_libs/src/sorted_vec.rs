/// Super simple vector that is maintained in a sorted state using a priority number
pub struct SortedVec<A: Clone>(pub Vec<A>, pub Vec<isize>);

impl <A: Clone>SortedVec<A> {
    /// Default empty vector
    pub fn default() -> Self {
        SortedVec(Vec::new(), Vec::new())
    }

    /// Get length of vector
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Insert an entry that is automatically sorted in
    pub fn insert(&mut self, entry: A, priority: isize) {
        let mut i = 0;
        for e in &self.1 {
            if priority < *e {
                break;
            }
            i += 1;
        }
        self.0.insert(i, entry.clone());
        self.1.insert(i, priority);
    }

    /// Remove a specific entry and return it
    pub fn remove(&mut self, index: usize) -> A {
        self.1.remove(index);
        self.0.remove(index)
    }

    /// Remove last entry and return it
    pub fn pop(&mut self) -> Option<A> {
        self.1.pop();
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_in_order() {
        let mut arr: SortedVec<usize> = SortedVec::default();

        arr.insert(0, 0);
        arr.insert(1, 0);
        arr.insert(2, 0);
        arr.insert(3, 0);

        assert_eq!(arr.0, [0, 1, 2, 3]);
    }

    #[test]
    fn insert_not_in_order() {
        let mut arr: SortedVec<usize> = SortedVec::default();

        arr.insert(0, 0);
        arr.insert(1, 2);
        arr.insert(2, 0);
        arr.insert(3, 1);

        assert_eq!(arr.0, [0, 2, 3, 1]);
    }

    #[test]
    fn insert_in_order_2() {
        let mut arr: SortedVec<usize> = SortedVec::default();

        arr.insert(3, 3);
        arr.insert(2, 2);
        arr.insert(1, 5);
        arr.insert(3, 0);

        assert_eq!(arr.0, [3, 2, 3, 1]);
    }

    #[test]
    fn insert_in_order_and_remove() {
        let mut arr: SortedVec<usize> = SortedVec::default();

        arr.insert(0, 0);
        arr.insert(1, 0);
        arr.insert(2, 0);
        arr.insert(3, 0);

        arr.remove(2);

        assert_eq!(arr.0, [0, 1, 3]);
    }
}
