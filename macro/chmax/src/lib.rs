// Reference: https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7

macro_rules! chmax {
    ($base:expr, $($ex:expr),+ $(,)*) => {
        let _chmax = max!($($ex),+);
        if $base < _chmax {
            $base = _chmax;
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
    ($exa:expr, $($ex:expr),+ $(,)*) => {
        std::cmp::max($exa, max!($($ex),+))
    };
}
