use std::cell::RefCell;
use std::io::{Read, StdoutLock, Write};

const BUF_SIZE: usize = 1 << 16; // flush the pending bytes once they reach this

// ---------------- input ----------------

pub trait Readable: Sized {
    fn read() -> Self;
}

pub fn token() -> &'static [u8] {
    SCANNER.with(|s| s.borrow_mut().get_or_insert_with(Scanner::new).token())
}

struct Scanner {
    buf: &'static [u8],
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut buf = vec![];
        std::io::stdin().read_to_end(&mut buf).unwrap();
        Self {
            buf: Box::leak(buf.into_boxed_slice()),
            pos: 0,
        }
    }

    fn token(&mut self) -> &'static [u8] {
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        let l = self.pos;
        while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        &self.buf[l..self.pos]
    }
}

thread_local! {
    static SCANNER: RefCell<Option<Scanner>> = const { RefCell::new(None) };
}

macro_rules! impl_readable_unsigned {
    ($($t:ty),*) => {
        $(
            impl Readable for $t {
                fn read() -> Self {
                    let t = token();
                    let mut v = 0;
                    for &c in t {
                        v = v * 10 + (c - b'0') as $t;
                    }
                    v
                }
            }
        )*
    };
}

macro_rules! impl_readable_signed {
    ($($t:ty),*) => {
        $(
            impl Readable for $t {
                fn read() -> Self {
                    let t = token();
                    let neg = t[0] == b'-';
                    let mut v = 0;
                    for &c in if neg { &t[1..] } else { t } {
                        v = v * 10 - (c - b'0') as $t;
                    }
                    if neg { v } else { -v }
                }
            }
        )*
    };
}

macro_rules! impl_readable_float {
    ($($t:ty),*) => {
        $(
            impl Readable for $t {
                fn read() -> Self {
                    std::str::from_utf8(token()).unwrap().parse().unwrap()
                }
            }
        )*
    };
}

macro_rules! impl_readable_tuple {
    ($($t:ident),*) => {
        impl<$($t: Readable),*> Readable for ($($t,)*) {
            fn read() -> Self {
                ($($t::read(),)*)
            }
        }
    };
}

impl_readable_unsigned!(u8, u16, u32, u64, u128, usize);
impl_readable_signed!(i8, i16, i32, i64, i128, isize);
impl_readable_float!(f32, f64);
impl_readable_tuple!(A, B);
impl_readable_tuple!(A, B, C);
impl_readable_tuple!(A, B, C, D);
impl_readable_tuple!(A, B, C, D, E);
impl_readable_tuple!(A, B, C, D, E, F);
impl_readable_tuple!(A, B, C, D, E, F, G);
impl_readable_tuple!(A, B, C, D, E, F, G, H);

impl Readable for char {
    fn read() -> Self {
        token()[0] as char
    }
}

impl Readable for String {
    fn read() -> Self {
        std::str::from_utf8(token()).unwrap().to_owned()
    }
}

#[macro_export]
macro_rules! input {
    ($($r:tt)*) => {
        $crate::input_inner!($($r)*);
    };
}

#[macro_export]
macro_rules! input_inner {
    () => {};
    (, $($r:tt)*) => {
        $crate::input_inner!($($r)*);
    };
    (mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = $crate::read_value!($t);
        $crate::input_inner!($($r)*);
    };
    ($var:ident : $t:tt $($r:tt)*) => {
        let $var = $crate::read_value!($t);
        $crate::input_inner!($($r)*);
    };
}

#[macro_export]
macro_rules! read_value {
    ([$t:tt; $len:expr]) => {
        (0..$len).map(|_| $crate::read_value!($t)).collect::<Vec<_>>()
    };
    ([$t:tt]) => {{
        let len = <usize as $crate::Readable>::read();
        (0..len).map(|_| $crate::read_value!($t)).collect::<Vec<_>>()
    }};
    (($($t:tt),*)) => {
        ($($crate::read_value!($t)),*)
    };
    (Chars) => {
        $crate::read_value!(String).chars().collect::<Vec<char>>()
    };
    (Bytes) => {
        $crate::token().to_vec()
    };
    (Usize1) => {
        <usize as $crate::Readable>::read() - 1
    };
    (Isize1) => {
        <isize as $crate::Readable>::read() - 1
    };
    ($t:ty) => {
        <$t as $crate::Readable>::read()
    };
}

#[macro_export]
macro_rules! define_query {
    (
        $name:ident {
            $( $tag:literal => $variant:ident ( $( $field:ident : $ty:ty ),* $(,)? ) ),* $(,)?
        }
    ) => {
        #[derive(Clone, Debug)]
        enum $name {
            $( $variant( $( $ty ),* ), )*
        }
        use $name::*;

        impl $crate::Readable for $name {
            fn read() -> Self {
                match <u32 as $crate::Readable>::read() {
                    $(
                        $tag => {
                            $crate::input! { $( $field: $ty ),* }
                            $variant( $( $field ),* )
                        }
                    )*
                    _ => unreachable!(),
                }
            }
        }
    };
}

// ---------------- output ----------------

pub trait Writable {
    fn write(&self, buf: &mut Vec<u8>);
}

pub struct Output {
    buf: Vec<u8>,
    out: StdoutLock<'static>,
}

impl Output {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(BUF_SIZE),
            out: std::io::stdout().lock(),
        }
    }

    pub fn print<T: Writable>(&mut self, v: T) {
        v.write(&mut self.buf);
        self.auto_flush();
    }

    pub fn println<T: Writable>(&mut self, v: T) {
        v.write(&mut self.buf);
        self.buf.push(b'\n');
        self.auto_flush();
    }

    pub fn print_iter<I>(&mut self, iter: I, sep: &str)
    where
        I: IntoIterator,
        I::Item: Writable,
    {
        for (i, v) in iter.into_iter().enumerate() {
            if i > 0 {
                self.buf.extend_from_slice(sep.as_bytes());
            }
            v.write(&mut self.buf);
        }
        self.auto_flush();
    }

    pub fn println_iter<I>(&mut self, iter: I, sep: &str)
    where
        I: IntoIterator,
        I::Item: Writable,
    {
        self.print_iter(iter, sep);
        self.buf.push(b'\n');
        self.auto_flush();
    }

    pub fn newline(&mut self) {
        self.buf.push(b'\n');
        self.auto_flush();
    }

    pub fn flush(&mut self) {
        self.out.write_all(&self.buf).unwrap();
        self.buf.clear();
        self.out.flush().unwrap();
    }

    fn auto_flush(&mut self) {
        if self.buf.len() >= BUF_SIZE {
            self.out.write_all(&self.buf).unwrap();
            self.buf.clear();
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        self.flush();
    }
}

impl Write for Output {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(b);
        self.auto_flush();
        Ok(b.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Output::flush(self);
        Ok(())
    }
}

macro_rules! impl_writable_unsigned {
    ($($t:ty),*) => {
        $(
            impl Writable for $t {
                fn write(&self, buf: &mut Vec<u8>) {
                    let mut v = *self;
                    if v == 0 {
                        buf.push(b'0');
                        return;
                    }
                    let mut tmp = [0; 39];
                    let mut i = tmp.len();
                    while v > 0 {
                        i -= 1;
                        tmp[i] = b'0' + (v % 10) as u8;
                        v /= 10;
                    }
                    buf.extend_from_slice(&tmp[i..]);
                }
            }
        )*
    };
}

macro_rules! impl_writable_signed {
    ($($t:ty),*) => {
        $(
            impl Writable for $t {
                fn write(&self, buf: &mut Vec<u8>) {
                    if *self < 0 {
                        buf.push(b'-');
                    }
                    self.unsigned_abs().write(buf);
                }
            }
        )*
    };
}

macro_rules! impl_writable_display {
    ($($t:ty),*) => {
        $(
            impl Writable for $t {
                fn write(&self, buf: &mut Vec<u8>) {
                    buf.extend_from_slice(self.to_string().as_bytes());
                }
            }
        )*
    };
}

impl_writable_unsigned!(u8, u16, u32, u64, u128, usize);
impl_writable_signed!(i8, i16, i32, i64, i128, isize);
impl_writable_display!(f32, f64);

impl Writable for str {
    fn write(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}

impl Writable for String {
    fn write(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}

impl Writable for char {
    fn write(&self, buf: &mut Vec<u8>) {
        let mut b = [0; 4];
        buf.extend_from_slice(self.encode_utf8(&mut b).as_bytes());
    }
}

impl<T: Writable + ?Sized> Writable for &T {
    fn write(&self, buf: &mut Vec<u8>) {
        (**self).write(buf);
    }
}

#[macro_export]
macro_rules! output {
    ($out:expr) => {
        $out.newline()
    };
    ($out:expr, $first:expr $(, $rest:expr)* $(,)?) => {{
        $out.print($first);
        $(
            $out.print(' ');
            $out.print($rest);
        )*
        $out.newline();
    }};
}

// ---------------- misc ----------------

pub trait Join: Iterator {
    fn join(self, sep: &str) -> String
    where
        Self: Sized,
        Self::Item: std::fmt::Display,
    {
        let mut s = String::new();
        for (i, item) in self.enumerate() {
            if i > 0 {
                s.push_str(sep);
            }
            s.push_str(&item.to_string());
        }
        s
    }
}

impl<I: Iterator> Join for I {}
