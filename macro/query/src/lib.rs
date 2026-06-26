#[macro_export]
macro_rules! define_query {
    (
        $name:ident {
            $( $tag:literal => $variant:ident ( $( $field:ident : $ty:ty ),* $(,)? ) ),* $(,)?
        }
    ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        enum $name {
            $( $variant( $( $ty ),* ), )*
        }
        use $name::*;

        impl proconio::source::Readable for $name {
            type Output = $name;
            fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
                match u32::read(source) {
                    $(
                        $tag => {
                            input! { from source, $( mut $field: $ty ),* }
                            $variant( $( $field ),* )
                        }
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}
