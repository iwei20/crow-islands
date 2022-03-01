use std::{ops::{Index, IndexMut}, fmt::Display};

pub trait ParallelGrid : Index<usize> + IndexMut<usize> + Sync + Display {
    type Item;
    fn at(&self, r: usize, c: usize) -> &Self::Item;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}