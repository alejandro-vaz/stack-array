//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(const_cmp)]
#![feature(const_destruct)]
#![feature(const_drop_in_place)]
#![feature(const_array)]
#![feature(const_try)]
#![feature(transmute_neo)]
#![feature(const_index)]
#![feature(const_range)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(const_closures)]
#![feature(const_trait_impl)]
#![feature(const_heap)]
#![feature(trusted_len)]
#![feature(const_clone)]
#![feature(new_range)]
#![feature(const_slice_make_iter)]
#![feature(generic_const_exprs)]
#![feature(const_iter)]
#![feature(const_convert)]
#![feature(const_default)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod comparisons;
mod conversions;
mod errors;
mod iterators;
mod references;

//> HEAD -> CORE
use core::{
    fmt::{
        Debug,
        Formatter,
        Result as Format
    }, 
    marker::Destruct, 
    mem::{
        MaybeUninit,
        forget,
        transmute_neo as transmute
    }, 
    ops::{
        Bound, 
        Drop, 
        RangeBounds
    }, 
    array::from_fn as arrayfn,
    ptr::copy,
    hint::unreachable_unchecked
};

//> HEAD -> CONSTRANGEITER
use constrangeiter::ConstIntoIterator;

//> HEAD -> ERRORS
pub use errors::{
    CapacityExceeded,
    UnmatchedCapacity
};


//^
//^ ARRAY
//^

//> ARRAY -> STRUCT
pub struct Array<Type, const N: usize> {
    length: usize,
    data: [MaybeUninit<Type>; N]
}

//> ARRAY -> IMPLEMENTATION
impl<Type, const N: usize> Array<Type, N> {
    pub const fn len(&self) -> usize {return self.length}
    pub const fn new() -> Self {return Self::default()}
    pub const fn is_full(&self) -> bool {return self.length == N}
    pub const fn repeat<const TIMES: usize>(self) -> Array<
        Type, 
        {TIMES * N}
    > where Type: [const] Clone + [const] Destruct, [(); TIMES * N]: {
        let (length, mut data) = self.into();
        let mut additional = MaybeUninit::<[Type; TIMES * N]>::uninit().transpose();
        if TIMES == 0 {for index in (0..length).const_into_iter() {
            unsafe {data[index].assume_init_drop();};
        }} else {
            for index in (0..length).const_into_iter() {
                additional[index].write(unsafe {data[index].assume_init_read()});
            }
            for iteration in (1..TIMES).const_into_iter() {
                for index in (0..length).const_into_iter() {
                    additional[index + length * iteration].write(unsafe {
                        data[index].assume_init_ref().clone()
                    });
                }
            }
        }
        return Array::from((length * TIMES, additional));
    }
    pub const fn resize<const M: usize>(
        self
    ) -> Array<Type, M> where Type: [const] Destruct {
        let (length, mut data) = self.into();
        let mut additional = MaybeUninit::<[Type; M]>::uninit().transpose();
        return if M >= length {
            for index in (0..length).const_into_iter() {
                additional[index].write(unsafe {data[index].assume_init_read()});
            }
            Array::from((length, additional))
        } else {
            for index in (0..M).const_into_iter() {
                additional[index].write(unsafe {data[index].assume_init_read()});
            }
            for index in (M..length).const_into_iter() {
                unsafe {data[index].assume_init_drop()};
            }
            Array::from((M, additional))
        }
    }
    pub const fn divide<const AT: usize>(self) -> (
        Array<Type, AT>, 
        Array<Type, {N - AT}>
    ) where [(); N - AT]: {
        let (length, data) = self.into();
        let (first, second) = unsafe {transmute(data)};
        return (Array {
            length: length.min(AT),
            data: first
        }, Array {
            length: length.saturating_sub(AT),
            data: second
        })
    }
    pub const fn join<const M: usize>(self, other: Array<Type, M>) -> Array<Type, {N + M}> {
        let (length, data) = self.into();
        let (slength, sdata) = other.into();
        let mut together = unsafe {transmute::<_, [MaybeUninit<Type>; N + M]>((data, sdata))};
        let pointer = together.as_mut_ptr();
        unsafe {copy(
            pointer.add(N),
            pointer.add(length),
            slength
        )}
        return Array {
            length: length + slength,
            data: together
        }
    }
    #[track_caller]
    pub const fn push(&mut self, value: Type) -> () {
        self.push_mut(value);
    }
    #[track_caller]
    pub const fn push_mut<'valid>(&'valid mut self, value: Type) -> &'valid mut Type {
        let reference = self.data[self.length].write(value);
        self.length += 1;
        return reference;
    }
    pub const fn pop(&mut self) -> Option<Type> {return if self.length == 0 {None} else {
        self.length -= 1;
        Some(unsafe {self.data[self.length].assume_init_read()})
    }}
    pub const fn pop_if(
        &mut self,
        decider: impl [const] FnOnce(&mut Type) -> bool + [const] Destruct
    ) -> Option<Type> {return if decider(self.last_mut()?) {self.pop()} else {None}}
    pub const fn clear(&mut self) -> () where Type: [const] Destruct {self.truncate(0)}
    pub const fn truncate(&mut self, length: usize) -> () where Type: [const] Destruct {
        for index in (length..self.length).const_into_iter() {
            unsafe {self.data.get_unchecked_mut(index).assume_init_drop()};
        }
        self.length = length;
    }
    #[track_caller]
    pub const fn insert(&mut self, index: usize, value: Type) -> () {
        self.insert_mut(index, value);
    }
    #[track_caller]
    pub const fn insert_mut<'valid>(
        &'valid mut self, 
        index: usize, 
        value: Type
    ) -> &'valid mut Type {
        assert!(index <= self.length, "tried to insert out of bounds");
        assert!(self.length != N, "array capacity exceeded");
        let pointer = unsafe {self.data.as_mut_ptr().add(index)};
        unsafe {copy(
            pointer,
            pointer.add(1),
            self.length - index
        )};
        let reference = unsafe {self.data.get_unchecked_mut(index).write(value)};
        self.length += 1;
        return reference;
    }
    #[track_caller]
    pub const fn remove(&mut self, index: usize) -> Type {
        assert!(index < self.length, "tried to remove out of bounds");
        let value = unsafe {self.data.get_unchecked(index).assume_init_read()};
        let pointer = unsafe {self.data.as_mut_ptr().add(index)};
        unsafe {copy(
            pointer.add(1),
            pointer,
            self.length - index - 1
        )};
        self.length -= 1;
        return value;
    }
    #[track_caller]
    pub const fn swap_remove(&mut self, index: usize) -> Type {
        assert!(index <= self.length - 1, "tried to remove out of bounds");
        let value = unsafe {self.data[index].assume_init_read()};
        self.data.swap(index, self.length - 1);
        self.length -= 1;
        return value;
    }
    pub const fn retain(
        &mut self, 
        mut closure: impl [const] FnMut(&mut Type) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {
        let mut offset = 0;
        for index in (0..self.length).const_into_iter() {
            let mut item = unsafe {self.data[index].assume_init_read()};
            match (closure(&mut item), offset == 0) {
                (true, true) => forget(item),
                (true, false) => {self.data[index - offset].write(item);},
                (false, _) => {
                    drop(item);
                    offset += 1;
                }
            }
        }
        self.length -= offset;
    }
    pub const fn dedup(
        &mut self
    ) -> () where Type: [const] PartialEq<Type> + [const] Destruct {self.dedup_by_key_with(
        const |element| element as *const Type,
        const |first, second| {
            unsafe {first.as_ref()}.unwrap() == unsafe {second.as_ref()}.unwrap()
        }
    )}
    pub const fn dedup_with(
        &mut self,
        mut decider: impl [const] FnMut(&mut Type, &mut Type) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {self.dedup_by_key_with(
        const |element| element as *mut Type, 
        const |first, second| decider(
            unsafe {first.as_mut()}.unwrap(), 
            unsafe {second.as_mut()}.unwrap()
        )
    )}
    pub const fn dedup_by_key<
        'valid, 
        Key: 'valid + [const] PartialEq<Key> + [const] Destruct
    >(
        &'valid mut self,
        transformation: impl [const] FnMut(&mut Type) -> Key + [const] Destruct
    ) -> () where Type: [const] Destruct {self.dedup_by_key_with(
        transformation, 
        const |first, second| first == second
    )}
    pub const fn dedup_by_key_with<'valid, Key: 'valid + [const] Destruct>(
        &'valid mut self, 
        mut transformation: impl [const] FnMut(&mut Type) -> Key + [const] Destruct,
        mut decider: impl [const] FnMut(&mut Key, &mut Key) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {
        if self.length == 0 {return}
        let mut offset = 0;
        let mut previous = transformation(unsafe {self.data[0].assume_init_mut()});
        for index in (1..self.length).const_into_iter() {
            let current = unsafe {self.data[index].assume_init_mut()};
            let mut key = transformation(current);
            if decider(&mut previous, &mut key) {
                unsafe {(current as *mut Type).drop_in_place()};
                drop(key);
                offset += 1;
            } else {
                previous = key;
                if offset != 0 {
                    let value = unsafe {(current as *mut Type).read()};
                    self.data[index - offset].write(value);
                }
            }
        }
        self.length -= offset;
    }
    pub const fn drain(
        &mut self, 
        range: impl [const] RangeBounds<usize> + [const] Destruct
    ) -> Self {
        let start = match range.start_bound() {
            Bound::Excluded(_) => unsafe {unreachable_unchecked()},
            Bound::Included(bound) => {
                assert!(*bound < self.length);
                *bound
            },
            Bound::Unbounded => 0
        };
        let end = match range.end_bound() {
            Bound::Excluded(bound) => {
                assert!(*bound <= self.length);
                *bound
            },
            Bound::Included(bound) => {
                assert!(*bound < self.length);
                *bound + 1
            },
            Bound::Unbounded => self.length
        };
        let mut additional = MaybeUninit::<[Type; N]>::uninit().transpose();
        let array = match end - start {
            0 => Array {
                length: 0,
                data: additional
            },
            1 => {
                additional[0].write(self.remove(start));
                Array {
                    length: 1,
                    data: additional
                }
            },
            amount => {
                for index in (start..end).const_into_iter() {
                    additional[index - start].write(unsafe {
                        self.data[index].assume_init_read()
                    });
                }
                for index in (end..self.length).const_into_iter() {
                    self.data[index - end + start].write(unsafe {
                        self.data[index].assume_init_read()
                    });
                }
                self.length -= end - start;
                Array {
                    length: amount,
                    data: additional
                }
            }
        };
        return array;
    }
}

//> ARRAY -> DROP
const impl<Type: [const] Destruct, const N: usize> Drop for Array<Type, N> {
    fn drop(&mut self) {self.clear()}
}

//> ARRAY -> DEBUG
impl<Type: Debug, const N: usize> Debug for Array<Type, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {
        return self.as_ref().fmt(formatter);
    }
}

//> ARRAY -> EXTEND
impl<Type, const N: usize> Extend<Type> for Array<Type, N> {
    fn extend<T: IntoIterator<Item = Type>>(&mut self, iter: T) {
        iter.into_iter().for_each(|item| self.push(item));
    }
}

//> ARRAY -> CLONE
const impl<Type: [const] Clone, const N: usize> Clone for Array<Type, N> {
    fn clone(&self) -> Self {return Array {
        length: self.length,
        data: arrayfn(const |index| if index >= self.length {MaybeUninit::uninit()} else {
            MaybeUninit::new(unsafe {self.data[index].assume_init_ref().clone()})
        })
    }}
}

//> ARRAY -> DEFAULT
const impl<Type, const N: usize> Default for Array<Type, N> {
    fn default() -> Self {return Self {
        data: MaybeUninit::uninit().transpose(),
        length: 0
    }}
}