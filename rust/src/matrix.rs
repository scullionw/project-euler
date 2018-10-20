use std::ops::{Index, IndexMut};
use std::slice::{Chunks, ChunksMut};
use std::mem;
use std::vec::IntoIter;

pub struct Matrix {
    data: Vec<u64>,
    rows: usize,
    cols: usize,
}

type RowSlice<'a> = Chunks<'a, u64>;
type RowSliceMut<'a> = ChunksMut<'a, u64>;

impl Matrix {
    // impl into and from
    // impl iterator over rows and cols
    // impl Deref with Target=[u64]
    // make generic for floats and ints
    // make generic for anything
    // row and column structs
    // iters in each dir
    // compare speed betweem Chunks and RowsMut
    // compare speed between row-major and Vec<Vec<u64>>
    pub fn from_square_slice(data: &[u64], dim: usize) -> Matrix {
        assert!(data.len() == dim * dim);

        Matrix {
            data: data.to_vec(),
            rows: dim,
            cols: dim
        }
    }

    pub fn row(&self, idx: usize) -> &[u64] {
        assert!(idx < self.rows);
        let begin = idx * self.cols;
        let end = begin + self.cols;
        &self.data[begin..end]
    }

    pub fn row_mut(&mut self, idx: usize) -> &mut [u64] {
        assert!(idx < self.rows);
        let begin = idx * self.cols;
        let end = begin + self.cols;
        &mut self.data[begin..end]
    }

    pub fn len_column(&self) -> usize {
        self.rows
    }

    pub fn len_row(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> Rows {
        Rows { current_row: 0, mat: self }
    }

    pub fn chunk_rows(&self) -> RowSlice {
        self.data.chunks(self.cols)
    }

    pub fn rows_mut(&mut self) -> RowsMut {
        RowsMut {
            current_row: 0,
            rows: self.rows,
            cols: self.cols,
            data: &mut self.data
        }
    }

    pub fn chunk_rows_mut(&mut self) -> RowSliceMut {
        self.data.chunks_mut(self.cols)
    }

    pub fn upper_diagonals(&self) -> UpperDiagonals {
        assert!(self.cols == self.rows); // specialize for square matrix only..
        let first_half = (0..self.cols) .map(|m| (0..=m).map(move |n| (m - n, n))
                                                        .map(|(row, col)| row*self.cols + col)
                                                        .collect::<Vec<_>>())
                                        .collect::<Vec<_>>();
                                
        let second_half = (0..self.cols).map(|m| (0..=m).map(move |n| ((self.cols - 1) - n, (self.cols - 1) - (m - n)))
                                                        .map(|(row, col)| row*self.cols + col)
                                                        .collect::<Vec<_>>())
                                        .rev()
                                        .skip(1)
                                        .collect::<Vec<_>>();
    
        let indexes = first_half.into_iter().chain(second_half.into_iter()).collect::<Vec<_>>().into_iter(); // make iterator instead

        UpperDiagonals { indexes, data: &self.data }
    }
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

impl<'a> Iterator for Rows<'a> {
    type Item = &'a [u64];

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current_row;
        if index != self.mat.rows {
            self.current_row += 1;
            Some(self.mat.row(index))
        } else {
            None
        }
    }
}

pub struct RowsMut<'a> {
    current_row: usize,
    rows: usize,
    cols: usize,
    data: &'a mut [u64],
}

impl<'a> Iterator for RowsMut<'a> {
    type Item = &'a mut [u64];

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current_row;
        if index != self.rows {
            let extracted = mem::replace(&mut self.data, &mut []);
            let (head, tail) = extracted.split_at_mut(self.cols);
            self.data = tail;
            self.current_row += 1;
            Some(head)
        } else {
            None
        }
    }
}

pub struct Diagonal<'a> {
    mat: &'a Matrix
}

pub struct UpperDiagonals<'a> {
    indexes: IntoIter<Vec<usize>>,
    data: &'a [u64]
}

impl<'a> Iterator for UpperDiagonals<'a> {
    type Item = Vec<&'a u64>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indexes.next() {
            Some(idxs) => {
                let view = self.data.iter()
                                    .enumerate()
                                    .filter(|(i, _)| idxs.contains(i))
                                    .map(|(_, x)| x)
                                    .collect::<Vec<&u64>>();
                Some(view)
            },
            None => None
        }
    }
}