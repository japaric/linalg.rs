use rand::distributions::range::Range;
use rand::task_rng;
use std::num::pow;
use super::super::test::Bencher;
use traits::MulAssign;
use vec;

// FIXME mozilla/rust#12249 DRYer benchmarks using macros
macro_rules! mul_assign {
    ($name:ident, $size:expr, $ty:ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let between = Range::new(0 as $ty, 1 as $ty);
            let mut rng = task_rng();
            let size = pow(10u, $size);

            let mut x = vec::rand(size, &between, &mut rng);
            let y = vec::rand(size, &between, &mut rng);

            b.iter(|| {
                x.mul_assign(&y)
            })
        }
    }
}

mul_assign!(f32_2, 2, f32)
mul_assign!(f32_3, 3, f32)
mul_assign!(f32_4, 4, f32)
mul_assign!(f32_5, 5, f32)
mul_assign!(f32_6, 6, f32)

mul_assign!(f64_2, 2, f64)
mul_assign!(f64_3, 3, f64)
mul_assign!(f64_4, 4, f64)
mul_assign!(f64_5, 5, f64)
mul_assign!(f64_6, 6, f64)
