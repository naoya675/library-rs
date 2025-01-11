macro_rules! debug {
    ($exa:expr $(,)*) => {
        #[cfg(debug_assertions)]
            eprintln!(concat!(stringify!($exa), " = {:?}"), &$exa);
        };
    ($exa:expr, $($ex:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
            eprintln!(concat!(stringify!($exa), " = {:?}",
                $(", ", stringify!($ex), " = {:?}" ),*), &$exa, $($ex),*);
        };
}
