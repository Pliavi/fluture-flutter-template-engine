#[macro_export]
macro_rules! guard_clause {
    ($cond:expr, $ret:expr) => {
        if $cond {
            return $ret;
        }
    };
}

extern crate pretty_assertions;

#[macro_export]
macro_rules! lexer_test {
    (FAIL: $name:ident, $func:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let func = $func;

            let got = func(src);
            pretty_assertions::assert_eq!(got.is_err(), true, "{:?} should be an error", got);
        }
    };
    ($name:ident, $func:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let should_be = TokenKind::from($should_be);
            let func = $func;

            let (got, _bytes_read) = func(src).unwrap();
            pretty_assertions::assert_eq!(got, should_be, "Input was {:?}", src);
        }
    };
}
