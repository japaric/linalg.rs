//! `for each stripe in m.vstripes[_mut](size)`
//!
//! Test that the iterator `stripe[:, col].iter()` is ordered and complete for any valid `size` and
//! `col`

#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate cast;
extern crate linalg;
extern crate quickcheck;
extern crate rand;

use cast::From;
use linalg::prelude::*;
use quickcheck::TestResult;

#[macro_use]
mod setup;

mod transposed {
    use cast::From;
    use linalg::prelude::*;
    use quickcheck::TestResult;

    #[quickcheck]
    fn submat(
        (srow, scol): (u32, u32),
        (nrows, ncols): (u32, u32),
        (size, row): (u32, u32),
    ) -> TestResult {
        enforce! {
            size > 0,
            row < nrows,
        };

        let m = ::setup::mat((srow + ncols, scol + nrows));
        let v = m.slice((srow.., scol..)).t();

        let mut i = 0;
        let mut stripes = v.vstripes(size);
        let mut count = usize::from(ncols / size + if ncols % size == 0 { 0 } else { 1 });

        while let Some(s) = stripes.next() {
            count -= 1;

            test_eq!(stripes.size_hint(), (count, Some(count)));

            for x in s.row(row) {
                test_eq!(x, &(srow + i, scol + row));

                i += 1;
            }
        }

        test_eq!(i, ncols)
    }

    #[quickcheck]
    fn submat_mut(
        (srow, scol): (u32, u32),
        (nrows, ncols): (u32, u32),
        (size, row): (u32, u32),
    ) -> TestResult {
        enforce! {
            size > 0,
            row < nrows,
        };

        let mut m = ::setup::mat((srow + ncols, scol + nrows));
        let mut v = m.slice_mut((srow.., scol..)).t();

        let mut i = 0;
        let mut stripes = v.vstripes_mut(size);
        let mut count = usize::from(ncols / size + if ncols % size == 0 { 0 } else { 1 });

        while let Some(mut s) = stripes.next() {
            count -= 1;

            test_eq!(stripes.size_hint(), (count, Some(count)));

            for x in s.row_mut(row) {
                test_eq!(x, &mut (srow + i, scol + row));

                i += 1;
            }
        }

        test_eq!(i, ncols)
    }
}

#[quickcheck]
fn submat(
    (srow, scol): (u32, u32),
    (nrows, ncols): (u32, u32),
    (size, row): (u32, u32),
) -> TestResult {
    enforce! {
        size > 0,
        row < nrows,
    };

    let m = ::setup::mat((srow + nrows, scol + ncols));
    let v = m.slice((srow.., scol..));

    let mut i = 0;
    let mut stripes = v.vstripes(size);
    let mut count = usize::from(ncols / size + if ncols % size == 0 { 0 } else { 1 });

    while let Some(s) = stripes.next() {
        count -= 1;

        test_eq!(stripes.size_hint(), (count, Some(count)));

        for x in s.row(row) {
            test_eq!(x, &(srow + row, scol + i));

            i += 1;
        }
    }

    test_eq!(i, ncols)
}

#[quickcheck]
fn submat_mut(
    (srow, scol): (u32, u32),
    (nrows, ncols): (u32, u32),
    (size, row): (u32, u32),
) -> TestResult {
    enforce! {
        size > 0,
        row < nrows,
    };

    let mut m = ::setup::mat((srow + nrows, scol + ncols));
    let mut v = m.slice_mut((srow.., scol..));

    let mut i = 0;
    let mut stripes = v.vstripes_mut(size);
    let mut count = usize::from(ncols / size + if ncols % size == 0 { 0 } else { 1 });

    while let Some(mut s) = stripes.next() {
        count -= 1;

        test_eq!(stripes.size_hint(), (count, Some(count)));

        for x in s.row_mut(row) {
            test_eq!(x, &mut (srow + row, scol + i));

            i += 1;
        }
    }

    test_eq!(i, ncols)
}
