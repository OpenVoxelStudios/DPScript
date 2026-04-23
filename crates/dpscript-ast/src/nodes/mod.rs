use thiserror::Error;

mod def;
mod expr;
mod meta;

#[derive(
    Debug, Clone, Copy, Error, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[error("expected variant did not match self!")]
pub struct TryIntoNodeError;

macro_rules! node_group {
    { $group: ident ($mod: path) = $($name: ident),*$(,)? } => {
        pastey::paste! {
            #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpanGroup)]
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
                    type Error = TryIntoNodeError;

                    fn try_into(self) -> Result<$mod::$name<'a>, Self::Error> {
                        match self {
                            Self::[<$name:upper_camel>](it) => Ok(it),
                            _ => Err(TryIntoNodeError),
                        }
                    }
                }
            )*
        }

        // impl<'a> std::fmt::Display for $name<'a> {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         match self {
        //             $($name::$variant(me) => write!(f, "{me}"),)*
        //         }
        //     }
        // }
    };
}

node_group! { Node (self) = }
