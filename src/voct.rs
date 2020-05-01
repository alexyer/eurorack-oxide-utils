//! V/Oct conversion methods.
#[allow(unused_imports)]
use micromath::F32Ext;

/// V/Oct type.
pub struct VOct(pub f32);

const C1_FREQ: f32 = 32.703;

impl VOct {
    /// Convert to Hz.
    pub fn hz(&self) -> f32 {
            C1_FREQ * 2_f32.powf(self.0 - 1.0)
    }
}