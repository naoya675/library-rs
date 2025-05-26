//! https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7

macro_rules! chmin {
    ($base:expr, $($ex:expr),+ $(,)*) => {
        let min = min!($($ex),+);
        if $base > min {
            $base = min;
            true
        } else {
            false
        }
    };
}

macro_rules! min {
    ($exa:expr $(,)*) => {
        $exa
    };
    ($exa:expr, $exb:expr $(,)*) => {
        std::cmp::min($exa, $exb)
    };
    ($exa:expr, $($ex:expr),+ $(,)*) => {
        std::cmp::min($exa, min!($($ex),+))
    };
}
