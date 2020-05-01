//! Utility library providing Eurorack related helper functions.
#![no_std]

pub mod voct;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    use super::voct::*;

    #[test]
    fn test_hz() {
        let cases = [
            (1.0, 32.703),
            (2.0, 65.406),
            (3.0, 130.812),
            (4.0, 261.624),
            (5.0, 523.248),
        ];

        for (v, freq) in &cases {
            let voct = VOct(*v);
            assert_eq!(voct.hz(), *freq);
        }


    }
}
