use assign::{DivAssign, MulAssign};
use complex::Complex;
use onezero::One;

use traits::SliceMut;
use {Mat, Transposed, SubMatMut};

macro_rules! scale {
    ($(($lhs:ty, $rhs:ty)),+,) => {
        $(
            // NOTE Secondary
            impl<'a> DivAssign<$rhs> for SubMatMut<'a, $lhs> {
                fn div_assign(&mut self, alpha: $rhs) {
                    let _1: $rhs = One::one();

                    self.mul_assign(_1 / alpha)
                }
            }

            // NOTE Secondary
            impl<'a> DivAssign<$rhs> for Transposed<SubMatMut<'a, $lhs>> {
                fn div_assign(&mut self, alpha: $rhs) {
                    self.0.div_assign(alpha)
                }
            }

            // NOTE Forward
            impl DivAssign<$rhs> for Transposed<Mat<$lhs>> {
                fn div_assign(&mut self, alpha: $rhs) {
                    self.slice_mut(..).div_assign(alpha)
                }
            }

            // NOTE Forward
            impl DivAssign<$rhs> for Mat<$lhs> {
                fn div_assign(&mut self, alpha: $rhs) {
                    self.slice_mut(..).div_assign(alpha)
                }
            }
         )+
    }
}

scale! {
    (Complex<f32>, Complex<f32>),
    (Complex<f32>, f32),
    (Complex<f64>, Complex<f64>),
    (Complex<f64>, f64),
    (f32, f32),
    (f64, f64),
}
