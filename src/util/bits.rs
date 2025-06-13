use miette::SourceSpan;

use super::Spanned;

pub trait HasBits {
    type Bit: Clone;

    fn get_bits(&self) -> Vec<Self::Bit>;
}

pub trait FromBits: HasBits {
    fn from_bits(bits: Vec<Self::Bit>) -> Self;
}

impl HasBits for String {
    type Bit = char;

    fn get_bits(&self) -> Vec<Self::Bit> {
        self.chars().collect()
    }
}

impl FromBits for String {
    fn from_bits(bits: Vec<Self::Bit>) -> Self {
        Self::from_iter(bits)
    }
}

impl<T: Clone> HasBits for Vec<T> {
    type Bit = T;

    fn get_bits(&self) -> Vec<Self::Bit> {
        self.clone()
    }
}

impl<T: Clone> FromBits for Vec<T> {
    fn from_bits(bits: Vec<Self::Bit>) -> Self {
        bits
    }
}

pub trait HasSpan {
    fn get_span(&self) -> SourceSpan;
}

impl<T> HasSpan for Spanned<T> {
    fn get_span(&self) -> SourceSpan {
        self.1
    }
}
