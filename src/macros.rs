#[macro_export]
macro_rules! string {
    ($x:expr) => {
        String::from($x)
    };
}