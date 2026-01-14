#[macro_export]
macro_rules! dbg_n {
    ($($t: tt)*) => {{
        dbg!($($t)*);
    }}
}

#[macro_export]
macro_rules! parse_err {
    ($cx: ident, $($t:tt)*) => {{
        return Err(miette::miette!($($t)*).with_source_code(miette::NamedSource::new($cx.file, $cx.code.to_string())));
    }};
}
