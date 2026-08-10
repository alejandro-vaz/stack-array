//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::{
    mem::MaybeUninit,
    slice::{
        Iter,
        IterMut
    },
    iter::{
        ExactSizeIterator,
        DoubleEndedIterator,
        FusedIterator,
        TrustedLen
    },
    marker::Destruct
};


//^
//^ ITERABLE
//^

//> ITERABLE -> STRUCT
pub struct Iterable<Type, const N: usize> {
    index: usize,
    reduced: usize,
    length: usize,
    data: [MaybeUninit<Type>; N]
} 

//> ITERABLE -> DROP
const impl<Type: [const] Destruct, const N: usize> Drop for Iterable<Type, N> {
    fn drop(&mut self) {while let Some(value) = self.next() {drop(value)}}
}

//> ITERABLE -> ITERATOR
const impl<Type, const N: usize> Iterator for Iterable<Type, N> {
    type Item = Type;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.length - self.reduced - self.index == 0 {None} else {
            let value = unsafe {self.data[self.index].assume_init_read()};
            self.index += 1;
            Some(value)
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        return (self.length - self.index, Some(self.length - self.index));
    }
}

//> ITERABLE -> EXACT SIZE
impl<Type, const N: usize> ExactSizeIterator for Iterable<Type, N> {}

//> ITERABLE -> DOUBLE ENDED
const impl<Type, const N: usize> DoubleEndedIterator for Iterable<Type, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        return if self.length - self.reduced - self.index == 0 {None} else {
            self.reduced += 1;
            Some(unsafe {self.data[self.length - self.reduced].assume_init_read()})
        }
    }
}

//> ITERABLE -> FUSED
impl<Type, const N: usize> FusedIterator for Iterable<Type, N> {}

//> ITERABLE -> TRUSTED LEN
unsafe impl<Type, const N: usize> TrustedLen for Iterable<Type, N> {}


//^
//^ ITERATORS
//^

//> ITERATORS -> FROM
impl<Type, const N: usize> FromIterator<Type> for Array<Type, N> {
    fn from_iter<T: IntoIterator<Item = Type>>(iter: T) -> Self {
        let mut array = Self::new();
        array.extend(iter);
        return array;
    }
}

//> ITERATORS -> INTO
const impl<Type, const N: usize> IntoIterator for Array<Type, N> {
    type Item = Type;
    type IntoIter = Iterable<Type, N>;
    fn into_iter(self) -> Self::IntoIter {
        let (length, data) = self.into();
        let iterator = Self::IntoIter {
            index: 0,
            reduced: 0,
            length: length,
            data: data
        };
        return iterator;
    }
}

//> ITERATORS -> BORROWED
const impl<'valid, Type, const N: usize> IntoIterator for &'valid Array<Type, N> {
    type Item = &'valid Type;
    type IntoIter = Iter<'valid, Type>;
    fn into_iter(self) -> Self::IntoIter {self.iter()}
}

//> ITERATORS -> BORROWED MUTABLY
const impl<'valid, Type, const N: usize> IntoIterator for &'valid mut Array<Type, N> {
    type Item = &'valid mut Type;
    type IntoIter = IterMut<'valid, Type>;
    fn into_iter(self) -> Self::IntoIter {self.iter_mut()}
}