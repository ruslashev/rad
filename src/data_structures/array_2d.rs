use std::ops::{Index, IndexMut};

pub struct Array2D<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Copy> Array2D<T> {
    pub fn new(init: T, width: usize, height: usize) -> Self {
        let data = vec![init; width * height];

        Self {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&T> {
        let idx = self.idx(x, y)?;
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut T> {
        let idx = self.idx(x, y)?;
        self.data.get_mut(idx)
    }

    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, y: usize) -> &Self::Output {
        assert!(y < self.height);

        let start = y * self.width;

        &self.data[start..start + self.width]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        assert!(y < self.height);

        let start = y * self.width;

        &mut self.data[start..start + self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let mut arr = Array2D::new(0, 3, 2);

        *arr.get_mut(1, 2).expect("unexpected out of bounds") = 123;

        assert_eq!(arr.get(1, 2), Some(&123));
        assert_eq!(arr.get(0, 0), Some(&0));
        assert_eq!(arr.get(2, 0), None);
    }

    #[test]
    fn via_trait() {
        let mut arr = Array2D::new(0, 3, 2);

        arr[1][2] = 123;

        assert_eq!(arr[1][2], 123);
        assert_eq!(arr[0][0], 0);
    }

    #[test]
    #[should_panic]
    fn trait_panic() {
        let arr = Array2D::new(0, 3, 2);
        let _x = arr[2][0];
    }
}