//! V/Oct conversion methods.

use core::ops;
#[allow(unused_imports)]
use micromath::F32Ext;

/// Trait to handle voltage per octave conversions.
pub trait Voltage {
    /// Voltage to Hertz.
    fn hz(&self) -> f32;

    /// Voltage to millisecond.
    fn ms(&self) -> u32;

    /// Voltage to microseconds.
    fn us(&self) -> u32;
}

/// V/Oct type.
#[derive(Debug)]
pub struct VOct(pub f32);

/// mV/Oct Type.
#[derive(Debug)]
pub struct MvOct(pub f32);

const C1_FREQ: f32 = 32.703;

impl Voltage for VOct {
    /// Convert to Hz.
    fn hz(&self) -> f32 {
        C1_FREQ * 2_f32.powf(self.0 - 1.0)
    }

    /// Convert to ms.
    fn ms(&self) -> u32 {
        (1000.0 / self.hz()) as u32
    }

    /// Convert to ms.
    fn us(&self) -> u32 {
        (1000000.0 / self.hz()) as u32
    }
}

impl Voltage for MvOct {
    fn hz(&self) -> f32{
        C1_FREQ * 2_f32.powf(self.0 / 1000.0 - 1.0)
    }

    fn ms(&self) -> u32{
        (1000.0 / self.hz()) as u32
    }

    fn us(&self) -> u32{
        (1000000.0 / self.hz()) as u32
    }
}

impl From<MvOct> for VOct {
    fn from(mv: MvOct) -> Self {
        Self(mv.0 / 1000.0)
    }
}

impl From<VOct> for MvOct {
    fn from(mv: VOct) -> Self {
        Self(mv.0 * 1000.0)
    }
}

/// Macro to implement arithmetic operations (e.g. multiplication, division)
/// for wrapper types.
macro_rules! impl_arithmetic {
    ($wrapper:ty, $wrapped:ty) => {
        impl ops::Mul<$wrapped> for $wrapper {
            type Output = Self;
            fn mul(self, rhs: $wrapped) -> Self {
                Self(self.0 * rhs)
            }
        }

        impl ops::MulAssign<$wrapped> for $wrapper {
            fn mul_assign(&mut self, rhs: $wrapped) {
                self.0 *= rhs;
            }
        }

        impl ops::Div<$wrapped> for $wrapper {
            type Output = Self;
            fn div(self, rhs: $wrapped) -> Self {
                Self(self.0 / rhs)
            }
        }

        impl ops::Div<$wrapper> for $wrapper {
            type Output = $wrapped;
            fn div(self, rhs: $wrapper) -> $wrapped {
                self.0 / rhs.0
            }
        }

        impl ops::DivAssign<$wrapped> for $wrapper {
            fn div_assign(&mut self, rhs: $wrapped) {
                self.0 /= rhs;
            }
        }

        impl ops::Add<$wrapped> for $wrapper {
            type Output = Self;
            fn add(self, rhs: $wrapped) -> Self {
                Self(self.0 + rhs)
            }
        }

        impl ops::Sub<$wrapped> for $wrapper {
            type Output = Self;
            fn sub(self, rhs: $wrapped) -> Self {
                Self(self.0 - rhs)
            }
        }

        impl ops::AddAssign<$wrapped> for $wrapper {
            fn add_assign(&mut self, rhs: $wrapped) {
                self.0 += rhs;
            }
        }

        impl ops::SubAssign<$wrapped> for $wrapper {
            fn sub_assign(&mut self, rhs: $wrapped) {
                self.0 -= rhs;
            }
        }
    };
}

impl_arithmetic!(VOct, f32);
impl_arithmetic!(MvOct, f32);
