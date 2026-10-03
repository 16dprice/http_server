#[macro_export]
macro_rules! basic_error {
    ($s:tt, $m:literal) => {
        impl fmt::Display for $s {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $m)
            }
        }

        impl error::Error for $s {}
    }
}
