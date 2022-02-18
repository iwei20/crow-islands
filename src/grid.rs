use std::{ops::{Index, IndexMut, Mul}};

use rayon::iter::{IntoParallelRefMutIterator, IndexedParallelIterator, IntoParallelRefIterator, IntoParallelIterator, ParallelIterator};

pub trait Grid : Index<usize> + IndexMut<usize> + Default {
    type Item;
    fn at(&self, r: usize, c: usize) -> &Self::Item;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

fn matmul(a: impl Grid<Item = f32>, b: impl Grid<Item = f32>) {

}

#[derive(Debug, Hash, Clone, Copy, Default)]
pub struct Const2D<T, const WIDTH: usize, const HEIGHT: usize> where T: Default + Copy {
    array: [[T; WIDTH]; HEIGHT]
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Index<usize> for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Grid for Const2D<T, WIDTH, HEIGHT> where T: Default + Copy{
    type Item = T;

    fn at(&self, r: usize, c: usize) -> &Self::Item {
        todo!()
    }
    fn get_width(&self) -> usize {
        todo!()
    }
    fn get_height(&self) -> usize {
        todo!()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const O_WIDTH: usize, const O_HEIGHT: usize> Mul<Const2D<f32, O_WIDTH, O_HEIGHT>> for Const2D<f32, WIDTH, HEIGHT> {
    type Output = Const2D<f32, O_WIDTH, HEIGHT>;
    fn mul(self, rhs: Const2D<f32, O_WIDTH, O_HEIGHT>) -> Self::Output {
        let mut result: Self::Output = Default::default();

        result.array.par_iter_mut().enumerate().for_each(|(r, row)| { 
            row.par_iter_mut().enumerate().for_each(|(c, ele)| {
                *ele = (0..self.get_width())
                        .into_par_iter()
                        .map(|index| self.at(r, index) * rhs.at(index, c))
                        .sum();
            })
        });

        result
    }
}

#[macro_export]
macro_rules! const_2d {
    ($r:expr, $w:expr) => {
        
    };
}


pub struct Dynamic2D {

}