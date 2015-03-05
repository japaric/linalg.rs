#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate linalg;
extern crate quickcheck;
extern crate rand;

#[macro_use]
mod setup;

mod col {
    use linalg::prelude::*;
    use quickcheck::TestResult;

    use setup;

    // Test that `iter().rev()` is correct for `ColVec`
    #[quickcheck]
    fn owned(size: usize) -> bool {
        setup::col(size).iter().rev().enumerate().all(|(i, &e)| {
            let i = size - i - 1;

            e == i
        })
    }

    // Test that `iter().rev()` is correct for `Col`
    #[quickcheck]
    fn slice((nrows, ncols): (usize, usize), col: usize) -> TestResult {
        enforce! {
            col < ncols,
        }

        test!({
            let m = setup::mat((nrows, ncols));
            let c = try!(m.col(col));
            let n = m.nrows();

            c.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (i, col)
            })
        })
    }

    // Test that `iter().rev()` is correct for `MutCol`
    #[quickcheck]
    fn slice_mut((nrows, ncols): (usize, usize), col: usize) -> TestResult {
        enforce! {
            col < ncols,
        }

        test!({
            let mut m = setup::mat((nrows, ncols));
            let n = m.nrows();
            let c = try!(m.col_mut(col));

            c.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (i, col)
            })
        })
    }

    // Test that `iter().rev()` is correct for `strided::Col`
    #[quickcheck]
    fn strided((nrows, ncols): (usize, usize), col: usize) -> TestResult {
        enforce! {
            col < ncols,
        }

        test!({
            let m = setup::mat((ncols, nrows)).t();
            let c = try!(m.col(col));
            let n = m.nrows();

            c.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (col, i)
            })
        })
    }

    // Test that `iter().rev()` is correct for `strided::MutCol`
    #[quickcheck]
    fn strided_mut((nrows, ncols): (usize, usize), col: usize) -> TestResult {
        enforce! {
            col < ncols,
        }

        test!({
            let mut m = setup::mat((ncols, nrows)).t();
            let n = m.nrows();
            let c = try!(m.col_mut(col));

            c.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (col, i)
            })
        })
    }
}

mod diag {
    use linalg::prelude::*;
    use quickcheck::TestResult;

    use setup;

    // Test that `iter().rev()` is correct for `Diag`
    #[quickcheck]
    fn strided(size: (usize, usize), diag: isize) -> TestResult {
        validate_diag!(diag, size);

        test!({
            let m = setup::mat(size);
            let d = try!(m.diag(diag));
            let n = d.len();

            if diag > 0 {
                d.iter().rev().enumerate().all(|(i, &e)| {
                    let i = n - i - 1;

                    e == (i, i + diag as usize)
                })
            } else {
                d.iter().rev().enumerate().all(|(i, &e)| {
                    let i = n - i - 1;

                    e == (i + (-diag as usize), i)
                })
            }
        })
    }

    // Test that `iter().rev()` is correct for `MutDiag`
    #[quickcheck]
    fn strided_mut(size: (usize, usize), diag: isize) -> TestResult {
        validate_diag!(diag, size);

        test!({
            let mut m = setup::mat(size);
            let d = try!(m.diag_mut(diag));
            let n = d.len();

            if diag > 0 {
                d.iter().rev().enumerate().all(|(i, &e)| {
                    let i = n - i - 1;

                    e == (i, i + diag as usize)
                })
            } else {
                d.iter().rev().enumerate().all(|(i, &e)| {
                    let i = n - i - 1;

                    e == (i + (-diag as usize), i)
                })
            }
        })
    }
}

mod row {
    use linalg::prelude::*;
    use quickcheck::TestResult;

    use setup;

    // Test that `iter().rev()` is correct for `RowVec`
    #[quickcheck]
    fn owned(size: usize) -> bool {
        setup::row(size).iter().rev().enumerate().all(|(i, &e)| {
            let i = size - i - 1;

            e == i
        })
    }

    // Test that `iter().rev()` is correct for `Row`
    #[quickcheck]
    fn slice((nrows, ncols): (usize, usize), row: usize) -> TestResult {
        enforce! {
            row < nrows,
        }

        test!({
            let m = setup::mat((ncols, nrows)).t();
            let r = try!(m.row(row));
            let n = m.ncols();

            r.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (i, row)
            })
        })
    }

    // Test that `iter().rev()` is correct for `MutRow`
    #[quickcheck]
    fn slice_mut((nrows, ncols): (usize, usize), row: usize) -> TestResult {
        enforce! {
            row < nrows,
        }

        test!({
            let mut m = setup::mat((ncols, nrows)).t();
            let n = m.ncols();
            let r = try!(m.row_mut(row));

            r.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (i, row)
            })
        })
    }

    // Test that `iter().rev()` is correct for `strided::Row`
    #[quickcheck]
    fn strided((nrows, ncols): (usize, usize), row: usize) -> TestResult {
        enforce! {
            row < nrows,
        }

        test!({
            let m = setup::mat((nrows, ncols));
            let r = try!(m.row(row));
            let n = m.ncols();

            r.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (row, i)
            })
        })
    }

    // Test that `iter().rev()` is correct for `strided::MutRow`
    #[quickcheck]
    fn strided_mut((nrows, ncols): (usize, usize), row: usize) -> TestResult {
        enforce! {
            row < nrows,
        }

        test!({
            let mut m = setup::mat((nrows, ncols));
            let n = m.ncols();
            let r = try!(m.row_mut(row));

            r.iter().rev().enumerate().all(|(i, &e)| {
                let i = n - i - 1;

                e == (row, i)
            })
        })
    }
}
