//! Test that:
//!
//! `(alpha * A * B + beta * C)[r, c] == alpha * A[r, :] * B[:, c] + beta * C[r, c]`
//!
//! where `C.nrows() == 1`
//!
//! for any valid `r`, `c`

#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate approx;
extern crate complex;
extern crate linalg;
extern crate quickcheck;
extern crate rand;

#[macro_use]
mod setup;

use complex::{c64, c128};
use linalg::prelude::*;
use quickcheck::TestResult;

// alpha * A^t * B + beta * C
mod transposed {
    use complex::{c64, c128};
    use linalg::prelude::*;
    use quickcheck::TestResult;

    macro_rules! tests {
        ($($t:ident),+) => {
            $(
                #[quickcheck]
                fn $t(
                    (srow, scol): (u32, u32),
                    (nrows, ncols): (u32, u32),
                    col: u32,
                ) -> TestResult {
                    enforce! {
                        nrows != 0,
                        col < ncols,
                    }

                    let alpha: $t = ::setup::rand::scalar();
                    let beta: $t = ::setup::rand::scalar();

                    let ref a = ::setup::rand::row(nrows);
                    let b = ::setup::rand::mat((srow + ncols, scol + nrows));
                    let b = b.slice((srow.., scol..)).t();
                    let c = ::setup::rand::row(ncols);

                    let e = c[col];
                    let c = alpha * a * b + beta * c;
                    let dot = a * b.col(col);

                    test_approx_eq! {
                        c[col],
                        alpha * dot + beta * e
                    }
                }
             )+
        }
    }

    tests!(f32, f64, c64, c128);
}

// alpha * A * B + beta * C
macro_rules! tests {
    ($($t:ident),+) => {
        $(
            #[quickcheck]
            fn $t((srow, scol): (u32, u32), (nrows, ncols): (u32, u32), col: u32) -> TestResult {
                enforce! {
                    nrows != 0,
                    col < ncols,
                }

                let alpha: $t = ::setup::rand::scalar();
                let beta: $t = ::setup::rand::scalar();

                let ref a = ::setup::rand::row(nrows);
                let b = ::setup::rand::mat((srow + nrows, scol + ncols));
                let b = b.slice((srow.., scol..));
                let c = ::setup::rand::row(ncols);

                let e = c[col];
                let c = alpha * a * b + beta * c;
                let dot = a * b.col(col);

                test_approx_eq! {
                    c[col],
                    alpha * dot + beta * e
                }
            }
         )+
    }
}

tests!(f32, f64, c64, c128);
