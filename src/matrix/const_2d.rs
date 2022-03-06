use std::{ops::{Mul, Index, IndexMut}, fmt::Display, iter::Sum, sync::{Mutex, Arc}};

use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, IndexedParallelIterator, ParallelIterator};

use super::{ParallelGrid, Dynamic2D};

#[derive(Debug, Hash, Clone)]
pub struct Const2D<T, const WIDTH: usize, const HEIGHT: usize> {
    array: [[T; WIDTH]; HEIGHT]
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Const2D<T, WIDTH, HEIGHT> where T: Copy {
    pub fn from(array: [[T; WIDTH]; HEIGHT]) -> Self {
        Self { array: array }
    }
    
    pub fn fill(item: T) -> Self {
        Self { array: [[item; WIDTH]; HEIGHT] }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Const2D<f64, WIDTH, HEIGHT> {
    pub fn ident() -> Self {
        debug_assert_eq!(WIDTH, HEIGHT, "Given size is not square");
        let mut result: Self = Default::default();
        let result_mutex = Arc::new(Mutex::new(&mut result));
        (0..WIDTH).into_par_iter().for_each(|i| {
            result_mutex.lock().expect("Identity mutex failed")[i][i] = 1f64;
        });
        result
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Default for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy {
    fn default() -> Self {
        Self { array: [[Default::default(); WIDTH]; HEIGHT] }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Index<usize> for Const2D<T, WIDTH, HEIGHT> {
    type Output = [T; WIDTH];

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Const2D<T, WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Display for Const2D<T, WIDTH, HEIGHT> where T: Sync + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (&self).into_iter().try_for_each(|row| -> std::fmt::Result { 
            row.iter().enumerate().try_for_each(|(c, ele)| -> std::fmt::Result {
                write!(f, "{}{}", ele, if c == self.get_width() - 1 {""} else {" "})
            })?;
            writeln!(f)
        })
    }
} 

impl<T, const WIDTH: usize, const HEIGHT: usize> ParallelGrid for Const2D<T, WIDTH, HEIGHT> where T: Sync + Display {
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

impl<T, const WIDTH: usize, const HEIGHT: usize> IntoIterator for Const2D<T, WIDTH, HEIGHT> {
    type Item = [T; WIDTH];
    type IntoIter = std::array::IntoIter<Self::Item, HEIGHT>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

impl<'data, T, const WIDTH: usize, const HEIGHT: usize> IntoIterator for &'data Const2D<T, WIDTH, HEIGHT> where T: 'data {
    type Item = &'data [T; WIDTH];
    type IntoIter = std::slice::Iter<'data, [T; WIDTH]>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.iter()
    }
}

impl<'data, T, const WIDTH: usize, const HEIGHT: usize> IntoIterator for &'data mut Const2D<T, WIDTH, HEIGHT> where T: 'data {
    type Item = &'data mut [T; WIDTH];
    type IntoIter = std::slice::IterMut<'data, [T; WIDTH]>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.iter_mut()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IntoParallelIterator for Const2D<T, WIDTH, HEIGHT> where T: Sync + Send {
    type Item = [T; WIDTH];
    type Iter = rayon::array::IntoIter<Self::Item, HEIGHT>;

    fn into_par_iter(self) -> Self::Iter {
        self.array.into_par_iter()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize, const O_WIDTH: usize, const O_HEIGHT: usize> Mul<Const2D<T, O_WIDTH, O_HEIGHT>> for Const2D<T, WIDTH, HEIGHT> where T: Default + Mul<Output = T> + Sync + Send + Sum + Copy + Display {
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

impl<T, const WIDTH: usize, const HEIGHT: usize> Mul<Dynamic2D<T>> for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy + Mul<Output = T> + Sync + Send + Sum + Display {
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