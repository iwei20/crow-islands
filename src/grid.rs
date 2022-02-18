use std::{ops::{Index, IndexMut, Mul}};

pub trait Grid : Index<usize> + IndexMut<usize> {
    type Item;
    fn get(&self, r: usize, c: usize) -> &Self::Item;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

fn matmul(a: impl Grid<Item = f32>, b: impl Grid<Item = f32>) {

}

#[derive(Debug, Hash, Clone)]
pub struct Const2D<T, const WIDTH: usize, const HEIGHT: usize> {
    array: [[T; WIDTH]; HEIGHT]
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Const2D<T, WIDTH, HEIGHT> {

}

impl<T, const WIDTH: usize, const HEIGHT: usize> Index<usize> for Const2D<T, WIDTH, HEIGHT> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Const2D<T, WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Grid for Const2D<T, WIDTH, HEIGHT> {
    type Item = T;

    fn get(&self, r: usize, c: usize) -> &Self::Item {
        todo!()
    }
    fn get_width(&self) -> usize {
        todo!()
    }
    fn get_height(&self) -> usize {
        todo!()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Mul for Const2D<T, WIDTH, HEIGHT> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
#[macro_export]
macro_rules! const_2d {
    ($r:expr, $w:expr) => {
        
    };
}


pub struct Dynamic2D {

}