# Array

`Array<Type, N>` is an stack-allocated vector of maximum capacity `N`, but not restricted to having an exact amount of elements as `[_; N]`.

## Usage

```rust
use stack_array::Array;

let mut array = Array::<u8, 5>::new();

array.push(0);
array.push(3);

array.extend([1, 2]);
```

The `Array` type exposes an API similar to that of `Vec` with common functions:
- `.push(Type) -> ()`,
- `.pop() -> Option<Type>`,
- `.extend(IntoIterator<Item = Type>) -> ()`,
- `.clear() -> ()`,
- `.insert(usize, Type) -> ()`,
- `.remove(usize) -> Type`,
- `.swap_remove(usize) -> Type`,
- `.retain(impl FnMut(&mut Type) -> bool) -> ()`,
- `.dedup() -> ()`,
- `.dedup_by(impl FnMut(&mut Type, &mut Type) -> bool) -> ()`,
- `.dedup_by_key<K: PartialEq>(impl FnMut(&mut Type) -> K) -> ()`,
- `.drain(impl RangeBounds<usize>) -> Self`

The type implements `Deref<Target = [Type]>` along with `DerefMut` to access the methods of the slice type. There are also specialized functions for resizing the array.

Furthermore, most of its methods and implementations use cutting-edge nightly const-features, which allows for complex compile-time constants:

```rust
#![allow(incomplete_features)]

#![feature(generic_const_exprs)]
#![feature(const_convert)]
#![feature(const_trait_impl)]

use stack_array::Array;

static ARRAY: Array<u8, 6> = const {
    let mut instance = Array::<u8, 6>::from([1, 2, 3]);
    instance.push(4);
    instance.insert(0, 0);
    instance.pop().unwrap();
    instance
};
```

## Performance

`Array` is stack-allocated, which means it does not make use of any allocator and maintains items inlined on the runtime stack or in the program memory.

Since data does not need to travel from RAM but rather stays on CPU cache, it is faster than a regular `Vec`.

## When to use this type

This type should be used when you need to store a finite and reasonable number of elements in a list, and you care about performance.