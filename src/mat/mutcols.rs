use Mat;
use traits::MatrixMutCols;

impl<'a, T> MatrixMutCols<'a> for Mat<T> {}

#[cfg(test)]
mod test {
    use quickcheck::TestResult;

    use test;
    use traits::{Iter, Matrix, MatrixMutCols};

    #[quickcheck]
    fn iter(size: (uint, uint)) -> TestResult {
        match test::mat(size) {
            None => TestResult::discard(),
            Some(mut m) => {
                TestResult::from_bool(m.mut_cols().enumerate().all(|(col, c)| {
                    c.iter().enumerate().all(|(row, e)| {
                        e.eq(&(row, col))
                    })
                }))
            },
        }
    }

    #[quickcheck]
    fn rev_iter(size: (uint, uint)) -> TestResult {
        match test::mat(size) {
            None => TestResult::discard(),
            Some(mut m) => {
                let ncols = m.ncols();

                TestResult::from_bool(m.mut_cols().rev().enumerate().all(|(col, c)| {
                    c.iter().enumerate().all(|(row, e)| {
                        e.eq(&(row, ncols - col - 1))
                    })
                }))
            },
        }
    }

    #[quickcheck]
    fn size_hint(size: (uint, uint), skip: uint) -> TestResult {
        match test::mat(size) {
            None => TestResult::discard(),
            Some(mut m) => {
                let ncols = m.ncols();

                if skip < ncols {
                    let hint = m.mut_cols().skip(skip).size_hint();

                    let left = ncols - skip;

                    TestResult::from_bool(hint == (left, Some(left)))
                } else {
                    TestResult::discard()
                }
            },
        }
    }

    macro_rules! sum {
        ($($ty:ident),+) => {$(
            mod $ty {
                use quickcheck::TestResult;
                use std::iter::AdditiveIterator as AI;

                #[allow(unused_imports)]
                use test::{c64, c128, mod};
                use traits::{Iter, MatrixMutCols, MatrixRows, SumCols};

                #[quickcheck]
                fn sum(size: (uint, uint), skip: uint) -> TestResult {
                    match test::rand_mat::<$ty>(size) {
                        None => TestResult::discard(),
                        Some(mut m) => {
                            let (_, ncols) = size;

                            if skip < ncols {
                                let sum = m.mut_cols().skip(skip).sum().unwrap();

                                TestResult::from_bool(sum.iter().zip(m.rows()).all(|(&e, r)| {
                                    // FIXME (rust-lang/rust#16949) Use static dispatch
                                    let ai = &mut r.iter().skip(skip).map(|&x| x) as &mut AI<$ty>;
                                    e == ai.sum()
                                }))
                            } else {
                                TestResult::discard()
                            }
                        }
                    }
                }
            }
        )+}
    }

    sum!(f32, f64, c64, c128)
}