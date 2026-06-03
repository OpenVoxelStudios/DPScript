mod macros {
    macro_rules! node_group {
        { $group: ident = [$($mod: ident::$name: ident),*$(,)?] } => {
            pastey::paste! {
                #[repr(u8)]
                #[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpanGroup)]
                pub enum $group<'a> {
                    $(
                        [<$name:upper_camel>]($mod::$name<'a>),
                    )*
                }

                impl<'a> $group<'a> {
                    $(
                        pub fn [<is_ $name:snake>](&self) -> bool {
                            match self {
                                Self::[<$name:upper_camel>](_) => true,
                                _ => false,
                            }
                        }

                        pub fn [<as_ $name:snake>](&self) -> Option<&$mod::$name<'a>> {
                            match self {
                                Self::[<$name:upper_camel>](it) => Some(it),
                                _ => None,
                            }
                        }

                        pub fn [<as_ $name:snake _mut>](&mut self) -> Option<&mut $mod::$name<'a>> {
                            match self {
                                Self::[<$name:upper_camel>](it) => Some(it),
                                _ => None,
                            }
                        }

                        pub fn [<into_ $name:snake>](self) -> Option<$mod::$name<'a>> {
                            match self {
                                Self::[<$name:upper_camel>](it) => Some(it),
                                _ => None,
                            }
                        }
                    )*
                }

                $(
                    impl<'a> Into<$group<'a>> for $mod::$name<'a> {
                        fn into(self) -> $group<'a> {
                            $group::[<$name:upper_camel>](self)
                        }
                    }

                    impl<'a> TryInto<$mod::$name<'a>> for $group<'a> {
                        type Error = $crate::nodes::err::TryIntoNodeError;

                        fn try_into(self) -> Result<$mod::$name<'a>, Self::Error> {
                            match self {
                                Self::[<$name:upper_camel>](it) => Ok(it),
                                _ => Err($crate::nodes::err::TryIntoNodeError),
                            }
                        }
                    }
                )*
            }

            // impl<'a> std::fmt::Display for $mod::$name<'a> {
            //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //         match self {
            //             $($name::$variant(me) => write!(f, "{me}"),)*
            //         }
            //     }
            // }
        };
    }

    pub(crate) use node_group;
}

pub(crate) use macros::*;
