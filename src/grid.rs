use std::ops::{Index, IndexMut};

pub trait Grid<T> : Index<usize> + IndexMut<usize> {

}

