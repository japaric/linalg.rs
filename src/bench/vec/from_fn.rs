use num::Complex;
use std::num;

use super::super::test::Bencher;
use vec;

// FIXME mozilla/rust#12249 DRYer benchmarks using macros
macro_rules! from_fn {
    ($name:ident, $size:expr, $ty: ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let size = num::pow(10u, $size);

            b.iter(|| {
                vec::from_fn(size, |i| i as $ty)
            })
        }
    }
}


from_fn!(f32_2, 2, f32)
from_fn!(f32_3, 3, f32)
from_fn!(f32_4, 4, f32)
from_fn!(f32_5, 5, f32)
from_fn!(f32_6, 6, f32)

from_fn!(f64_2, 2, f64)
from_fn!(f64_3, 3, f64)
from_fn!(f64_4, 4, f64)
from_fn!(f64_5, 5, f64)
from_fn!(f64_6, 6, f64)

macro_rules! from_fn_complex {
    ($name:ident, $size:expr, $ty: ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let size = num::pow(10u, $size);

            b.iter(|| {
                vec::from_fn(size, |i| Complex::new(i as $ty, i as $ty))
            })
        }
    }
}

from_fn_complex!(c64_2, 2, f32)
from_fn_complex!(c64_3, 3, f32)
from_fn_complex!(c64_4, 4, f32)
from_fn_complex!(c64_5, 5, f32)
from_fn_complex!(c64_6, 6, f32)

from_fn_complex!(c128_2, 2, f64)
from_fn_complex!(c128_3, 3, f64)
from_fn_complex!(c128_4, 4, f64)
from_fn_complex!(c128_5, 5, f64)
from_fn_complex!(c128_6, 6, f64)
