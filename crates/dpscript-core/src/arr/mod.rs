mod facet_impl;

use core::fmt;
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct DynArray<T> {
    inner: NonNull<T>,
    len: usize,
}

unsafe impl<T> Send for DynArray<T> {}
unsafe impl<T> Sync for DynArray<T> {}

impl<T> DynArray<T> {
    pub fn from_array(arr: Box<[T]>) -> Self {
        let len = arr.len();
        let ptr = Box::leak(arr).as_mut_ptr();
        let inner = unsafe { NonNull::new_unchecked(ptr) };

        Self { inner, len }
    }

    pub fn into_inner(self) -> Box<[T]> {
        let slice = std::ptr::slice_from_raw_parts_mut(self.inner.as_ptr(), self.len);

        unsafe { Box::from_raw(slice) }
    }
}

impl<T> Deref for DynArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { &*std::ptr::slice_from_raw_parts(self.inner.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for DynArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *std::ptr::slice_from_raw_parts_mut(self.inner.as_ptr(), self.len) }
    }
}

impl<T: Hash> Hash for DynArray<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        <[T] as Hash>::hash(&self, state)
    }
}

impl<T: PartialEq> PartialEq for DynArray<T> {
    fn eq(&self, other: &Self) -> bool {
        <[T] as PartialEq>::eq(&self, other)
    }

    fn ne(&self, other: &Self) -> bool {
        <[T] as PartialEq>::ne(&self, other)
    }
}

impl<T: Eq> Eq for DynArray<T> {}

impl<T: PartialOrd> PartialOrd for DynArray<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        <[T] as PartialOrd>::partial_cmp(&self, other)
    }

    fn lt(&self, other: &Self) -> bool {
        <[T] as PartialOrd>::lt(&self, other)
    }

    fn le(&self, other: &Self) -> bool {
        <[T] as PartialOrd>::le(&self, other)
    }

    fn gt(&self, other: &Self) -> bool {
        <[T] as PartialOrd>::gt(&self, other)
    }

    fn ge(&self, other: &Self) -> bool {
        <[T] as PartialOrd>::ge(&self, other)
    }
}

impl<T: Ord> Ord for DynArray<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        <[T] as Ord>::cmp(&self, other)
    }
}

impl<T: Serialize> Serialize for DynArray<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        <[T] as Serialize>::serialize(&self, serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for DynArray<T>
where
    Vec<T>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Vec<T> as Deserialize<'de>>::deserialize(deserializer)
            .map(|it| Self::from_array(it.into_boxed_slice()))
    }
}

impl<'a, T> IntoIterator for &'a DynArray<T>
where
    &'a [T]: IntoIterator,
{
    type Item = <&'a [T] as IntoIterator>::Item;
    type IntoIter = <&'a [T] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        <&'a [T] as IntoIterator>::into_iter(self)
    }
}

impl<T> fmt::Debug for DynArray<T>
where
    [T]: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
