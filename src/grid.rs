use std::{ops::{Index, IndexMut, Mul}, iter::Sum};

use rayon::iter::{IntoParallelRefMutIterator, IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

pub trait Grid : Index<usize> + IndexMut<usize> {
    type Item;
    fn at(&self, r: usize, c: usize) -> &Self::Item;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct Const2D<T, const WIDTH: usize, const HEIGHT: usize> where T: Default + Copy {
    array: [[T; WIDTH]; HEIGHT]
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Const2D<T, WIDTH, HEIGHT> where T: Default + Copy {
    pub fn fill(item: T) -> Self {
        Self { array: [[item; WIDTH]; HEIGHT] }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Default for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy {
    fn default() -> Self {
        Self { array: [[Default::default(); WIDTH]; HEIGHT] }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Index<usize> for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy {
    type Output = [T; WIDTH];

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Grid for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy{
    type Item = T;

    fn at(&self, r: usize, c: usize) -> &Self::Item {
        &self[r][c]
    }
    fn get_width(&self) -> usize {
        WIDTH
    }
    fn get_height(&self) -> usize {
        HEIGHT
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize, const O_WIDTH: usize, const O_HEIGHT: usize> Mul<Const2D<T, O_WIDTH, O_HEIGHT>> for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy + Mul<Output = T> + Sync + Send + Sum {
    type Output = Const2D<T, O_WIDTH, HEIGHT>;
    fn mul(self, rhs: Const2D<T, O_WIDTH, O_HEIGHT>) -> Self::Output {
        let mut result: Self::Output = Default::default();

        result.array.par_iter_mut().enumerate().for_each(|(r, row)| { 
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

impl<T, const WIDTH: usize, const HEIGHT: usize> Mul<Dynamic2D<T>> for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy + Mul<Output = T> + Sync + Send + Sum {
    type Output = Dynamic2D<T>;
    fn mul(self, rhs: Dynamic2D<T>) -> Self::Output {
        let mut result: Self::Output = Dynamic2D::new(rhs.get_width(), self.get_height());

        result.array.par_iter_mut().enumerate().for_each(|(r, row)| { 
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

#[macro_export]
macro_rules! const_2d {
    [$t:ty; $w:expr, $h:expr] => {
        Const2D<$t, $w, $h>::new()
    };
    [$item:expr, $t:ty; $w:expr, $h:expr] => {
        Const2D<$t, $w, $h>::fill($item)
    };
}

#[derive(Clone, Debug)]
pub struct Dynamic2D<T> where T: Default + Copy {
    width: usize,
    height: usize,
    array: Vec<Vec<T>>
}

impl<T> Dynamic2D<T> where T: Default + Copy {
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
}

impl<T> Index<usize> for Dynamic2D<T> where T: Default + Copy {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T> IndexMut<usize> for Dynamic2D<T> where T: Default + Copy {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T> Grid for Dynamic2D<T> where T: Default + Copy {
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