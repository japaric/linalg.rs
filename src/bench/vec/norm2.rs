use num::Complex;
use rand::distributions::{IndependentSample,Range};
use std::{num,rand};

use array::traits::ArrayNorm2;
use super::super::test::Bencher;
use vec;

// FIXME mozilla/rust#12249 DRYer benchmarks using macros
macro_rules! norm2 {
    ($name:ident, $size:expr, $ty:ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let between = Range::<$ty>::new(num::zero(), num::one());
            let mut rng = rand::task_rng();
            let size = num::pow(10u, $size);

            let v = vec::rand(size, &between, &mut rng);

            b.iter(|| {
                v.norm2()
            })
        }
    }
}

norm2!(f32_2, 2, f32)
norm2!(f32_3, 3, f32)
norm2!(f32_4, 4, f32)
norm2!(f32_5, 5, f32)
norm2!(f32_6, 6, f32)

norm2!(f64_2, 2, f64)
norm2!(f64_3, 3, f64)
norm2!(f64_4, 4, f64)
norm2!(f64_5, 5, f64)
norm2!(f64_6, 6, f64)

macro_rules! norm2_complex {
    ($name:ident, $size:expr, $ty:ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let between = Range::<$ty>::new(num::zero(), num::one());
            let mut rng = rand::task_rng();
            let size = num::pow(10u, $size);

            let v = vec::from_fn(size, |_| {
                Complex::new(between.ind_sample(&mut rng),
                           between.ind_sample(&mut rng))
            });

            b.iter(|| {
                v.norm2()
            })
        }
    }
}

norm2_complex!(c64_2, 2, f32)
norm2_complex!(c64_3, 3, f32)
norm2_complex!(c64_4, 4, f32)
norm2_complex!(c64_5, 5, f32)
norm2_complex!(c64_6, 6, f32)

norm2_complex!(c128_2, 2, f64)
norm2_complex!(c128_3, 3, f64)
norm2_complex!(c128_4, 4, f64)
norm2_complex!(c128_5, 5, f64)
norm2_complex!(c128_6, 6, f64)
