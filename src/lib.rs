//! Utility library providing Eurorack related helper functions.
#![no_std]

pub mod voct;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    use super::voct::*;

    #[test]
    fn test_voct_hz() {
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

    #[test]
    fn test_mvoct_hz() {
        let cases = [
            (1000.0, 32.703),
            (2000.0, 65.406),
            (3000.0, 130.812),
            (4000.0, 261.624),
            (5000.0, 523.248),
        ];

        for (v, freq) in &cases {
            let voct = MvOct(*v);
            assert_eq!(voct.hz(), *freq);
        }
    }

    #[test]
    fn test_voct_ms() {
        let cases = [(1.0, 30), (2.0, 15), (3.0, 7), (4.0, 3), (5.0, 1)];

        for (v, period) in &cases {
            let voct = VOct(*v);
            assert_eq!(voct.ms(), *period);
        }
    }

    #[test]
    fn test_voct_us() {
        let cases = [
            (1.0, 30578),
            (2.0, 15289),
            (3.0, 7644),
            (4.0, 3822),
            (5.0, 1911),
        ];

        for (v, period) in &cases {
            let voct = VOct(*v);
            assert_eq!(voct.us(), *period);
        }
    }

    #[test]
    fn test_mvoct_ms() {
        let cases = [
            (1000.0, 30),
            (2000.0, 15),
            (3000.0, 7),
            (4000.0, 3),
            (5000.0, 1),
        ];

        for (v, period) in &cases {
            let voct = MvOct(*v);
            assert_eq!(voct.ms(), *period);
        }
    }

    #[test]
    fn test_mvoct_us() {
        let cases = [
            (1000.0, 30578),
            (2000.0, 15289),
            (3000.0, 7644),
            (4000.0, 3822),
            (5000.0, 1911),
        ];

        for (v, freq) in &cases {
            let voct = MvOct(*v);
            assert_eq!(voct.us(), *freq);
        }
    }
}
