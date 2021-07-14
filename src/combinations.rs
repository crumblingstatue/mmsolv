//! A modified version of the mmpw type. Doesn't use const generics.
//! Apparently permutations is the wrong term.

use std::{borrow::Borrow, marker::PhantomData};

pub struct SliceCombo<T, S> {
    slice: S,
    indices: Box<[usize]>,
    first: bool,
    _p: PhantomData<T>,
}

impl<T, S> SliceCombo<T, S> {
    pub fn new(slice: S, slots: usize) -> Self {
        Self {
            slice,
            indices: vec![0; slots].into_boxed_slice(),
            first: true,
            _p: PhantomData,
        }
    }
}

impl<T: Clone, S: Borrow<[T]>> Iterator for SliceCombo<T, S> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let slice = self.slice.borrow();
        if slice.is_empty() {
            return None;
        }
        let len = self.indices.len();
        let mut arr = vec![slice[0].clone(); len];
        if self.first {
            self.first = false;
            return Some(arr);
        }
        let mut i = len - 1;
        loop {
            if self.indices[i] < slice.len() - 1 {
                self.indices[i] += 1;
                for (j, &indice) in self.indices.iter().enumerate() {
                    arr[j] = slice[indice].clone();
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
