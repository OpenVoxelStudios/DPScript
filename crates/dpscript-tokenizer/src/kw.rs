macro_rules! keywords {
    {
        $($name: ident $(= $real: expr)?;)*
    } => {
        pastey::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
            pub enum Keyword {
                $($name),*
            }

            impl std::fmt::Display for Keyword {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.as_str())
                }
            }

            impl Keyword {
                const fn to_char_array<const N: usize>(s: &'static str) -> [char; N] {
                    let mut arr = [' '; N];
                    let mut pos = 0;
                    let b = s.as_bytes();

                    loop {
                        if pos >= N {
                            break;
                        }

                        arr[pos] = b[pos] as char;
                        pos += 1;
                    }

                    arr
                }

                pub const fn as_str(&self) -> &'static str {
                    match self {
                        $(
                            Self::$name => keywords!(one; $name $(= $real)?)
                        ),*
                    }
                }

                #[allow(non_upper_case_globals)]
                pub fn try_parse<'a>(cur: char, iter: &mut dpscript_core::StringCursor<'a>) -> Option<Self> {
                    $(
                        const [<_KW_ $name _RAW>]: &str = keywords!(one; $name $(= $real)?);
                        const [<_KW_ $name _LEN>]: usize = [<_KW_ $name _RAW>].len();
                        const [<_KW_ $name _PEEK>]: usize = [<_KW_ $name _LEN>] - 1;
                        const [<_KW_ $name _CHARS>]: [char; [<_KW_ $name _LEN>]] = Keyword::to_char_array([<_KW_ $name _RAW>]);
                    )*

                    $(
                        let c0 = [<_KW_ $name _CHARS>][0];

                        if cur == c0 && (
                            [<_KW_ $name _LEN>] == 1
                            || iter.peek_many([<_KW_ $name _PEEK>]).is_some_and(|it| it == &[<_KW_ $name _RAW>][1..])
                        ) && iter.peek().is_none_or(|it| !it.is_alphanumeric()) {
                            if [<_KW_ $name _LEN>] != 1 {
                                let _ = iter.take([<_KW_ $name _PEEK>]);
                            }

                            return Some(Self::$name);
                        }

                        iter.clear_peeker();
                    )*

                    None
                }
            }
        }
    };

    (one; $name: ident = $real: expr) => {
        $real
    };

    (one; $name: ident) => {
        pastey::paste! { stringify!([<$name:lower>]) }
    };
}

keywords! {
    // Top-level declarations
    Import;
    Export;
    Const;
    Fn;
    Objective;
    Struct; // type check only
    Enum;
    Init;
    Tick;

    // Modifiers
    Pub;
    Operator;
    Ref; // `ref arg: type`
    Extends; // `struct A extends B`
    Typedef;

    // Expressions
    Let;
    Return;
    Break;
    Continue;

    // Blocks
    As;
    At;
    If;
    Else;
    For;
    In;
    While;
    Loop; // infinite loop

    // Values
    True;
    False;
}
