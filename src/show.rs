use std::fmt::{Formatter, Show, mod};
use traits::{Iter, MatrixRows};
use {Mat, MutView, Trans, View};

// XXX Sadly, I can't implement `Show` generically, so I'll repeat myself using macros

// TODO Precision, padding
macro_rules! fmt {
    () => {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            for row in self.rows() {
                try!(write!(f, "["));

                let mut is_first = true;
                for e in row.iter() {
                    if is_first {
                        is_first = false;
                    } else {
                        try!(write!(f, ", "));
                    }
                    try!(write!(f, "{}", e));
                }

                try!(writeln!(f, "]"))
            }

            Ok(())
        }
    }
}

impl<T: Show> Show for Mat<T> { fmt!() }
impl<T: Show> Show for Trans<Mat<T>> { fmt!() }

macro_rules! impl_show {
    ($($ty:ty),+) => {$(
        impl<'a, T: Show> Show for $ty {
            fmt!()
        }
    )+}
}

impl_show!(MutView<'a, T>, Trans<MutView<'a, T>>, Trans<View<'a, T>>, View<'a, T>)
