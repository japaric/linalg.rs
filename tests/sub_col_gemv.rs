//! Test that:
//!
//! - `(alpha * A * B - beta * C)[r, c] == alpha * A[r, :] * B[:, c] - beta * C[r, c]`
//! - `(beta * C - alpha * A * B)[r, c] == beta * C[r, c] - alpha * A[r, :] * B[:, c]`
//!
//! where `C.ncols() == 1`
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

// alpha * A^t * B - beta * C
// beta * C - alpha * A^t * B
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
                    row: u32,
                    reverse: bool,
                ) -> TestResult {
                    enforce! {
                        ncols != 0,
                        row < nrows,
                    }

                    let alpha: $t = ::setup::rand::scalar();
                    let beta: $t = ::setup::rand::scalar();

                    let a = ::setup::rand::mat((srow + ncols, scol + nrows));
                    let a = a.slice((srow.., scol..)).t();
                    let ref b = ::setup::rand::col(ncols);
                    let c = ::setup::rand::col(nrows);

                    let e = c[row];
                    let dot = a.row(row) * b;

                    if reverse {
                        let c = beta * c - alpha * a * b;

                        test_approx_eq! {
                            c[row],
                            beta * e - alpha * dot
                        }
                    } else {
                        let c = alpha * a * b - beta * c;

                        test_approx_eq! {
                            c[row],
                            alpha * dot - beta * e
                        }
                    }
                }
             )+
        }
    }

    tests!(f32, f64, c64, c128);
}

// alpha * A * B - beta * C
// beta * C - alpha * A * B
macro_rules! tests {
    ($($t:ident),+) => {
        $(
            #[quickcheck]
            fn $t(
                (srow, scol): (u32, u32),
                (nrows, ncols): (u32, u32),
                row: u32,
                reverse: bool,
            ) -> TestResult {
                enforce! {
                    ncols != 0,
                    row < nrows,
                }

                let alpha: $t = ::setup::rand::scalar();
                let beta: $t = ::setup::rand::scalar();

                let a = ::setup::rand::mat((srow + nrows, scol + ncols));
                let a = a.slice((srow.., scol..));
                let ref b = ::setup::rand::col(ncols);
                let c = ::setup::rand::col(nrows);

                let e = c[row];
                let dot = a.row(row) * b;

                if reverse {
                    let c = beta * c - alpha * a * b;

                    test_approx_eq! {
                        c[row],
                        beta * e - alpha * dot
                    }
                } else {
                    let c = alpha * a * b - beta * c;

                    test_approx_eq! {
                        c[row],
                        alpha * dot - beta * e
                    }
                }
            }
         )+
    }
}

tests!(f32, f64, c64, c128);
