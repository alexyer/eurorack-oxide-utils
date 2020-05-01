//! V/Oct conversion methods.

/// V/Oct type.
pub struct VOct(pub u16);

const C1_FREQ: f32 = 32.703;

impl VOct {
    /// Convert to Hz.
    pub fn hz(&self) -> f32 {
            C1_FREQ * 2_u16.pow(self.0 as u32 - 1) as f32
    }
}