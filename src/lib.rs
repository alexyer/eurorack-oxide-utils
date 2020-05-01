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
            (1, 32.703),
            (2, 65.406),
            (3, 130.812),
            (4, 261.624),
            (5, 523.248),
        ];

        for (v, freq) in &cases {
            let voct = VOct(*v);
            assert_eq!(voct.hz(), *freq);
        }


    }
}
