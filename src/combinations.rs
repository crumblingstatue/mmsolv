//! A modified version of the mmpw type. Doesn't use const generics.
//! Apparently permutations is the wrong term.

pub struct SliceCombo<'a, T> {
    slice: &'a [T],
    indices: Box<[usize]>,
    first: bool,
}

impl<'a, T> SliceCombo<'a, T> {
    pub fn new(slice: &'a [T], slots: usize) -> Self {
        Self {
            slice,
            indices: vec![0; slots].into_boxed_slice(),
            first: true,
        }
    }
}

impl<'a, T: Clone> Iterator for SliceCombo<'a, T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        let len = self.indices.len();
        let mut arr = vec![self.slice[0].clone(); len];
        if self.first {
            self.first = false;
            return Some(arr);
        }
        let mut i = len - 1;
        loop {
            if self.indices[i] < self.slice.len() - 1 {
                self.indices[i] += 1;
                for (j, &indice) in self.indices.iter().enumerate() {
                    arr[j] = self.slice[indice].clone();
                }
                return Some(arr);
            } else {
                self.indices[i] = 0;
                if i == 0 {
                    return None;
                }
                i -= 1;
            }
        }
    }
}
