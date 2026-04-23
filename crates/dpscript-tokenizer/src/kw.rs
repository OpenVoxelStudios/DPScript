use crate::util::PeekSized;

macro_rules! keywords {
    {
        $($name: ident $(= $real: expr)?;)*
    } => {
        pastey::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum Keyword {
                $($name),*
            }

            impl Keyword {
                pub const fn as_str(&self) -> &'static str {
                    match self {
                        $(
                            Self::$name => keywords!(one; $name $(= $real)?)
                        ),*
                    }
                }

                #[allow(non_upper_case_globals)]
                pub fn try_parse<I: Iterator<Item = char>>(
                    cur: char,
                    iter: &mut peekmore::PeekMoreIterator<I>,
                ) -> Option<Self> {
                    $(
                        const [<_KW_ $name _RAW>]: &str = keywords!(one; $name $(= $real)?);
                        const [<_KW_ $name _LEN>]: usize = [<_KW_ $name _RAW>].len();
                        const [<_KW_ $name _CHARS>]: [char; [<_KW_ $name _LEN>]] = const_str::to_char_array!(keywords!(one; $name $(= $real)?));
                    )*

                    $(
                        if cur == [<_KW_ $name _CHARS>][0] && (
                            [<_KW_ $name _LEN>] == 1
                            || iter.peek_many::<[<_KW_ $name _LEN>]>().is_some_and(|it| it == [<_KW_ $name _CHARS>][1..])
                        ) {
                            if [<_KW_ $name _LEN>] != 1 {
                                let _ = iter.take([<_KW_ $name _LEN>] - 1);
                            }

                            return Some(Self::$name);
                        }
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
        stringify!(pastey::paste! { [<$name:lower>] })
    };
}

keywords! {
    // Top-level declarations
    Import;
    Export;
    Const;
    Fn;
    Objective;
    Field;
    Struct; // type check only
    Enum;

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
    For;
    While;
    Loop; // infinite loop

    // Values
    True;
    False;
}
