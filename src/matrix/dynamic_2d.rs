use std::{ops::{Index, IndexMut}, fmt::Display};
use rayon::iter::{IntoParallelRefMutIterator, IndexedParallelIterator, IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};

use super::ParallelGrid;

#[derive(Clone, Debug)]
pub struct Dynamic2D<T> {
    width: usize,
    height: usize,
    array: Vec<Vec<T>>
}

impl<T> Dynamic2D<T> where T: Default + Clone + Sync + Send + Display {
    pub fn new(width: usize, height: usize) -> Self {
        Dynamic2D::fill(Default::default(), width, height)
    }

    pub fn fill(item: T, width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            array: vec![vec![item; width]; height]
        }
    }

    pub fn add_row(&mut self, items: impl IndexedParallelIterator<Item = T>) {
        debug_assert_eq!(items.len(), self.get_width());
        self.array.push(items.collect());
    }

    pub fn add_col(&mut self, items: impl IndexedParallelIterator<Item = T>) {
        debug_assert_eq!(items.len(), self.get_height());
        self.array.par_iter_mut().zip(items).for_each(|(vec, item)| {
            vec.push(item);
        })
    }
}

impl<T> Index<usize> for Dynamic2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T> IndexMut<usize> for Dynamic2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T> Display for Dynamic2D<T> where T: Sync + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (&self).into_iter().try_for_each(|row| -> std::fmt::Result { 
            row.iter().enumerate().try_for_each(|(c, ele)| -> std::fmt::Result {
                write!(f, "{} ", ele)
            })?;
            writeln!(f)
        })
    }
} 

impl<T> ParallelGrid for Dynamic2D<T> where T: Sync + Display {
    type Item = T;

    fn at(&self, r: usize, c: usize) -> &Self::Item {
        &self[r][c]
    }
    fn get_width(&self) -> usize {
        self.width
    }
    fn get_height(&self) -> usize {
        self.height
    }
}

impl<T> IntoIterator for Dynamic2D<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

impl<'data, T> IntoIterator for &'data Dynamic2D<T> where T: 'data {
    type Item = &'data Vec<T>;
    type IntoIter = std::slice::Iter<'data, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.iter()
    }
}

impl<'data, T> IntoIterator for &'data mut Dynamic2D<T> where T: 'data {
    type Item = &'data mut Vec<T>;
    type IntoIter = std::slice::IterMut<'data, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.iter_mut()
    }
}

impl<T> IntoParallelIterator for Dynamic2D<T> where T: Send {
    type Item = Vec<T>;
    type Iter = rayon::vec::IntoIter<Self::Item>;

    fn into_par_iter(self) -> Self::Iter {
        self.array.into_par_iter()
    }
}

impl<'data, T> IntoParallelIterator for &'data Dynamic2D<T> where T: 'data + Sync {
    type Item = &'data Vec<T>;
    type Iter = rayon::slice::Iter<'data, Vec<T>>;

    fn into_par_iter(self) -> Self::Iter {
        self.array.par_iter()
    }
}

impl<'data, T> IntoParallelIterator for &'data mut Dynamic2D<T> where T: 'data + Send {
    type Item = &'data mut Vec<T>;
    type Iter = rayon::slice::IterMut<'data, Vec<T>>;

    fn into_par_iter(self) -> Self::Iter {
        self.array.par_iter_mut()
    }
}

impl<T> Mul<Dynamic2D<T>> for Dynamic2D<T> where T: Default + Copy + Mul<Output = T> + Sync + Send + Sum + Display + std::fmt::Debug {
    type Output = Dynamic2D<T>;
    fn mul(self, rhs: Dynamic2D<T>) -> Self::Output {
        let mut result: Self::Output = Dynamic2D::new(rhs.get_width(), self.get_height());

        result.par_iter_mut().enumerate().for_each(|(r, row)| { 
            row.par_iter_mut().enumerate().for_each(|(c, ele)| {
                *ele = (0..self.get_width())
                        .into_par_iter()
                        .map(|index| *(self.at(r, index)) * *(rhs.at(index, c)))
                        .sum();
            })
        });
        
        result
    }
}