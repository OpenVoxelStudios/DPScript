#[macro_export]
macro_rules! cmd_enum {
    {
        $(#[doc = $doc: expr])?
        pub enum $name: ident {
            $(
                $(#[doc = $vdoc: expr])?
                #[print = $fmt: expr]
                $var: ident {
                    $(
                        $(#[doc = $fdoc: expr])?
                        $field: ident: $ty: ty
                    ),*
                    $(,)?
                }
            ),*
            $(,)?
        }
    } => {
        #[derive(Debug, Clone)]
        $(#[doc = $doc])?
        pub enum $name {
            $(
                $(#[doc = $vdoc])?
                $var {
                    $(
                        $(#[doc = $fdoc])?
                        $field: $ty
                    ),*
                }
            ),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, #[allow(unused)] f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$var { $($field),* } => write!(f, $fmt),
                    )*

                    #[allow(unused)]
                    _ => unreachable!()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! cmd_struct {
    {
        $(#[doc = $doc: expr])?
        #[print = $fmt: expr]
        pub struct $name: ident {
            $(
                $(#[doc = $fdoc: expr])?
                $field: ident: $ty: ty
            ),*
            $(,)?
        }
    } => {
        #[derive(Debug, Clone)]
        $(#[doc = $doc])?
        pub struct $name {
            $(
                $(#[doc = $fdoc])?
                pub $field: $ty
            ),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let Self { $($field),* } = self;

                write!(f, $fmt)
            }
        }
    };
}

#[macro_export]
macro_rules! cmd_enums {
    {
        $(
            $(#[enum_doc = $doc: expr])?
            pub enum $name: ident {
                $($t: tt)*
            }
        )*

        $(
            $(#[struct_doc = $s_doc: expr])?
            #[print = $fmt: expr]
            pub struct $s_name: ident {
                $($s_t: tt)*
            }
        )*
    } => {
        $($crate::cmd_enum! { $(#[doc = $doc])? pub enum $name { $($t)* } })*
        $($crate::cmd_struct! { $(#[doc = $s_doc])? #[print = $fmt] pub struct $s_name { $($s_t)* } })*
    };
}
