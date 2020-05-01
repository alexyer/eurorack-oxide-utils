/// Conditional debug
#[macro_export]
macro_rules! cdbg {
    ($($x:expr), *) => {
        if cfg!(debug_assertions) {
            use cortex_m_semihosting::dbg;
            dbg!($($x),*);
        }
    };
}