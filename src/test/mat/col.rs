use quickcheck::TestResult;

use mat;
use mat::traits::{MatrixCol,MatrixColIterator};
use traits::Iterable;

// FIXME rust-lang/rust#15734 Replace `index` method with `[]` operator
#[quickcheck]
fn index(shape@(nrows, ncols): (uint, uint), col: uint) -> TestResult {
    if col >= ncols {
        return TestResult::discard();
    }

    let m = mat::from_fn(shape, |r, c| (r, c));
    let m_col = m.col(col);
    let mut rows = range(0, nrows);

    TestResult::from_bool(rows.all(|row| m_col.index(&row) == &(row, col)))
}

// FIXME rust-lang/rust#15734 Replace `index` method with `[]` operator
#[quickcheck]
fn iterable(shape@(_, ncols): (uint, uint), col: uint) -> TestResult {
    if col >= ncols {
        return TestResult::discard();
    }

    let m = mat::from_fn(shape, |r, c| (r, c));
    let m_col = m.col(col);

    TestResult::from_bool(m_col.iter().enumerate().all(|(r, e)| m.index(&(r, col)) == e))
}

// FIXME rust-lang/rust#15734 Replace `index` method with `[]` operator
#[quickcheck]
fn iterator(shape: (uint, uint)) -> bool {
    let m = mat::from_fn(shape, |r, c| (r, c));

    m.cols().enumerate().all(|(c, col)| {
        col.iter().enumerate().all(|(r, e)| m.index(&(r, c)) == e)
    })
}
