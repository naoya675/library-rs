macro_rules! chmax {
    ($base:expr, $($ex:expr),+ $(,)*) => {
        let max = max!($($ex),+);
        if $base < max {
            $base = max;
            true
        } else {
            false
        }
    };
}

macro_rules! max {
    ($exa:expr $(,)*) => {
        $exa
    };
    ($exa:expr, $exb:expr $(,)*) => {
        std::cmp::max($exa, $exb)
    };
    ($exa:expr, $(ex:expr),+ $(,)*) => {
        std::cmp::max($exa, max!($($ex),+))
    };
}
