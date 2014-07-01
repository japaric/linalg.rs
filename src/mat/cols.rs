use std::mem;

use mat::Col;
use mat::traits::{MatrixCol,MatrixShape};

// TODO mozilla/rust#13302 Enforce Copy on M
pub struct Cols<M> {
    mat: M,
    state: uint,
    stop: uint,
}

impl<
    M: Copy + MatrixShape
> Cols<M> {
    #[inline]
    pub fn new(mat: M) -> Cols<M> {
        Cols {
            mat: mat,
            state: 0,
            stop: mat.ncols(),
        }
    }
}

impl <
    M: Copy + MatrixCol
> Iterator<Col<M>>
for Cols<M> {
    #[inline]
    fn next(&mut self) -> Option<Col<M>> {
        let state = self.state;

        if state < self.stop {
            Some(unsafe {
                self.mat.unsafe_col(mem::replace(&mut self.state, state + 1))
            })
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        let exact = self.stop - self.state;

        (exact, Some(exact))
    }
}
