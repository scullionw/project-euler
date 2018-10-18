use std::ops::{Index,IndexMut};

// Bench and compare version with Vec<Vec<u64>>
pub struct Matrix {
    data: Vec<u64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    // impl into and from
    // impl iterator over rows and cols
    // impl Deref with Target=[u64]
    // make generic for floats and ints
    // make generic for anything
    // row and column structs
    // iters in each dir
    pub fn from_square_slice(data: &[u64], dim: usize) -> Matrix {
        assert!(data.len() == dim * dim);

        Matrix {
            data: data.to_vec(),
            rows: dim,
            cols: dim
        }
    }

    pub fn row(&self, idx: usize) -> &[u64]{
        assert!(idx < self.rows);
        let begin = idx * self.cols;
        let end = begin + self.cols;
        &self.data[begin..end]
    }

    pub fn row_mut(&mut self, idx: usize) -> &mut [u64]{
        assert!(idx < self.rows);
        let begin = idx * self.cols;
        let end = begin + self.cols;
        &mut self.data[begin..end]
    }

    pub fn height(&self) -> usize {
        self.rows
    }

    pub fn width(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> Rows {
        Rows { current_row: 0, mat: &self } // maybe just self
    }

    // pub fn rows_mut(&mut self) -> RowsMut {
    //     RowsMut { current_row: 0, mat: &mut self }
    // }
}

impl Index<(usize, usize)> for Matrix {
    type Output = u64;

    fn index(&self, index: (usize, usize)) -> &u64 {
        assert!(index.0 < self.rows && index.1 < self.cols);
        &self.data[index.0 * self.cols + index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {

    fn index_mut(&mut self, index: (usize, usize)) -> &mut u64 {
        assert!(index.0 < self.rows && index.1 < self.cols);
        &mut self.data[index.0 * self.cols + index.1]
    }
}

impl Index<usize> for Matrix {
    type Output = [u64];

    fn index(&self, row: usize) -> &[u64] {
        assert!(row < self.rows);
        let begin = row * self.cols;
        let end = begin + self.cols;
        &self.data[begin..end]
    }
}

impl IndexMut<usize> for Matrix {

    fn index_mut(&mut self, row: usize) -> &mut [u64] {
        assert!(row < self.rows);
        let begin = row * self.cols;
        let end = begin + self.cols;
        &mut self.data[begin..end]
    }
}


pub struct Rows<'a> {
    current_row: usize,
    mat: &'a Matrix
}

// make mut version?
impl<'a> Iterator for Rows<'a> {
    type Item = &'a [u64];

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current_row;
        if index != (self.mat.rows - 1) {
            self.current_row += 1;
            Some(self.mat.row(index))
        } else {
            None
        }
    }
}

// pub struct RowsMut<'a> {
//     current_row: usize,
//     mat: &'a mut Matrix
// }

// impl<'a> Iterator for RowsMut<'a> {
//     type Item = &'a mut [u64];

//     fn next(&mut self) -> Option<Self::Item> {
//         let index = self.current_row;
//         if index != (self.mat.rows - 1) {
//             self.current_row += 1;
//             Some(self.mat.row_mut(index))
//         } else {
//             None
//         }
//     }
// }