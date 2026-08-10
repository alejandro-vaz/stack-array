//^
//^ HEAD
//^

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(generic_const_exprs)]

//> HEAD -> STACK_ARRAY
use stack_array::Array;


//^
//^ TESTS
//^

//> TESTS -> PUSHPOP
#[test]
fn pushpop() -> () {
    let mut new = Array::<u8, 5>::new();
    new.push(0);
    new.push(1);
    new.push(2);
    new.push(3);
    new.push(4);
    new.pop();
    new.push(5);
    assert_eq!(new.as_ref(), &[0, 1, 2, 3, 5]);
}

//> TESTS -> PUSHPANIC
#[test]
#[should_panic]
fn pushpanic() -> () {
    let mut new = Array::<u8, 1>::new();
    new.push(0);
    new.push(1);
}

//> TESTS -> NONEPOP
#[test]
fn nonepop() -> () {
    let mut new = Array::<u8, 1>::from([2]);
    assert_eq!(new.pop(), Some(2));
    assert_eq!(None, new.pop());
}

//> TESTS -> LEN
#[test]
fn len() -> () {
    let mut new = Array::<u8, 5>::new();
    new.push(0);
    new.push(1);
    assert_eq!(new.len(), 2);
    new.pop();
    assert_eq!(new.len(), 1);
    new.extend([1, 2, 3]);
    assert_eq!(new.len(), 4);
}

//> TESTS -> CLEAR
#[test]
fn clear() -> () {
    let mut new = Array::<u8, 5>::from([1, 2, 3]);
    new.clear();
    assert_eq!(new.len(), 0);
}

//> TESTS -> DROP
#[test]
#[should_panic]
fn drop() -> () {
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            panic!();
        }
    }
    let _ = Array::<Guard, 1>::from([Guard]);
}

//> TESTS -> GET
#[test]
fn get() -> () {
    let mut array = Array::<u8, 3>::from([1, 2, 3]);
    assert_eq!(Some(&1), array.get(0));
    assert_eq!(Some(&mut 2), array.get_mut(1));
    assert_eq!(None, array.get(4));
}

//> TESTS -> INTOITER
#[test]
fn intoiter() -> () {
    let initial = [1, 2, 3];
    let array = Array::<u8, 3>::from(initial.clone());
    assert_eq!(
        array.into_iter().collect::<Vec<u8>>(), 
        initial.into_iter().collect::<Vec<u8>>()
    );
}

//> TESTS -> ITER
#[test]
fn iter() -> () {
    let initial = [1, 2, 3];
    let array = Array::<u8, 3>::from(initial.clone());
    assert_eq!(array.iter().collect::<Vec<&u8>>(), initial.iter().collect::<Vec<&u8>>());
}

//> TESTS -> EQ
#[test]
fn eq() -> () {
    let initial = [1, 2, 3];
    let array = Array::<u8, 3>::from(initial.clone());
    assert_eq!(array.as_ref(), initial);
}

//> TESTS -> ORD
#[test]
fn ord() -> () {
    let initial = [1, 2, 3];
    let array = Array::<u8, 3>::from(initial.clone());
    assert_eq!(array.as_ref().cmp(&initial), initial.as_slice().cmp(array.as_ref()));
}

//> TESTS -> INSERT
#[test]
fn insert() -> () {
    let mut array = Array::<u8, 6>::from([1, 2, 3]);
    array.insert(0, 0);
    assert_eq!(array.as_ref(), [0, 1, 2, 3]);
    array.insert(1, 4);
    assert_eq!(array.as_ref(), [0, 4, 1, 2, 3]);
    array.insert(5, 0);
    assert_eq!(array.as_ref(), [0, 4, 1, 2, 3, 0]);
}

//> TESTS -> INSERTNOLENGTH
#[test]
#[should_panic]
fn insertnolength() -> () {
    let mut array = Array::<u8, 6>::from([1, 2, 3]);
    array.insert(5, 0);
}

//> TESTS -> INSERTNOCAP
#[test]
#[should_panic]
fn insertnocap() -> () {
    let mut array = Array::<u8, 3>::from([1, 2, 3]);
    array.insert(3, 0);
}

//> TESTS -> REMOVE
#[test]
fn remove() -> () {
    let mut array = Array::<u8, 3>::from([1, 2, 3]);
    array.remove(1);
    assert_eq!(array.as_ref(), [1, 3]);
    array.remove(0);
    assert_eq!(array.as_ref(), [3]);
}

//> TESTS -> REMOVENOLENGTH
#[test]
#[should_panic]
fn removenolength() -> () {
    let mut array = Array::<u8, 5>::new();
    array.remove(0);
}

//> TESTS -> REMOVENOCAP
#[test]
#[should_panic]
fn removenocap() -> () {
    let mut array = Array::<u8, 7>::from([1, 2, 3]);
    array.remove(5);
}

//> TESTS -> INDEX
#[test]
fn index() -> () {
    let x = Array::<usize, 5>::from([0]);
    let _ = x[0];
}

//> TESTS -> BORROWED ITERATOR
#[test]
fn biter() -> () {
    let mut array = Array::<u8, 7>::from([1, 2, 3]);
    for element in &array {
        let _ = *element;
    }
    for element in &mut array {
        *element = *element + 1;
    }
}

//> TESTS -> INTOVEC
#[test]
fn intovec() -> () {
    let array = Array::<u8, 7>::from([1, 2, 3]);
    let vector: Vec<_> = array.into();
    assert_eq!(vector, [1, 2, 3]);
}

//> TESTS -> ZST
#[test]
fn zst() -> () {
    let mut array = Array::<(), 4>::from([(), (), ()]);
    array.pop();
    assert_eq!(array.len(), 2);
    array.extend([()].repeat(2));
    assert_eq!(array.len(), 4);
}

//> TESTS -> MUTABLES
#[test]
fn mutables() -> () {
    let mut array = Array::<u8, 7>::from([1, 2, 3]);
    let x = array.push_mut(6);
    assert_eq!(x, &mut 6);
    array.insert_mut(2, 3);
    assert_eq!(array.get(2), Some(&3));
}

//> TESTS -> RETAIN
#[test]
fn retain() -> () {
    let mut array = Array::<u8, 7>::from([1, 2, 3, 4, 5, 6]);
    array.retain(|value| value.is_power_of_two());
    assert_eq!(array.as_ref(), &[1, 2, 4]);
}

//> TESTS -> INDEX
#[test]
fn indexto() -> () {
    let array = Array::<u8, 7>::from([1, 2, 3, 4, 5, 6]);
    assert_eq!(array[2], 3);
}

//> TESTS -> DEDUP
#[test]
fn dedup() -> () {
    let mut array = Array::<u8, 10>::from([0, 4, 1, 2, 3, 0, 0, 3]);
    array.sort();
    array.dedup();
    assert_eq!(array, [0, 1, 2, 3, 4]);
    array.clear();
    array.dedup();
}

//> TESTS -> SWAPREMOVE
#[test]
fn swapremove() -> () {
    let mut array = Array::<u8, 10>::from([0, 4, 1, 2, 3, 0, 0, 3]);
    array.swap_remove(2);
    assert_eq!(array, [0, 4, 3, 2, 3, 0, 0]);
}

//> TESTS -> REPEAT
#[test]
fn repeat() -> () {
    let array = Array::<u8, 7>::from([1, 2, 3]);
    let double = array.repeat::<2>();
    assert_eq!(double, [1, 2, 3, 1, 2, 3]);
    let same = double.repeat::<1>();
    assert_eq!(same, [1, 2, 3, 1, 2, 3]);
    let none = same.repeat::<0>();
    assert_eq!(none, []);
}

//> TESTS -> RESIZE
#[test]
fn resize() -> () {
    let array = Array::<u8, 7>::from([1, 2, 3]);
    let same = array.resize::<4>();
    assert_eq!(same, [1, 2, 3]);
    let trimmed = same.resize::<2>();
    assert_eq!(trimmed, [1, 2]);
}

//> TESTS -> DRAIN
#[test]
fn drain() -> () {
    let mut array = Array::<u8, 10>::from([0, 4, 1, 2, 3, 0, 0, 3]);
    let subarray = array.drain(2..=5);
    assert_eq!(subarray, [1, 2, 3, 0]);
    assert_eq!(array, [0, 4, 0, 3]);
    let last = array.drain(3..);
    assert_eq!(last, [3]);
    assert_eq!(array, [0, 4, 0]);
    assert_eq!(array.len(), 3);
    let more = array.drain(2..=2);
    assert_eq!(more, [0]);
    let once = array.drain(..1);
    assert_eq!(once, [0]);
}

//> TESTS -> DIVIDE
#[test]
fn divide() -> () {
    let array = Array::<u8, 10>::from([0, 4, 1, 2, 3, 0, 0, 3]);
    let (first, second) = array.divide::<4>();
    assert_eq!(first, [0, 4, 1, 2]);
    assert_eq!(second, [3, 0, 0, 3]);
    let (zero, some) = first.divide::<0>();
    assert_eq!(zero, []);
    assert_eq!(some, [0, 4, 1, 2]);
    let (reverse, null) = second.divide::<6>();
    assert_eq!(reverse, [3, 0, 0, 3]);
    assert_eq!(null, []);
}

//> TESTS -> JOIN
#[test]
fn join() -> () {
    let first = Array::<u8, 3>::from([1, 2, 3]);
    let second = Array::<u8, 7>::from([0, 1]);
    let third = first.join(second);
    assert_eq!(third, [1, 2, 3, 0, 1]);
}

//> TESTS -> INTOFIXED
#[test]
fn intofixed() -> () {
    let array = Array::<u8, 7>::from([1, 2, 3]);
    let now: [u8; 3] = array.clone().try_into().unwrap();
    assert_eq!(now, [1, 2, 3]);
    let small = array.resize::<3>();
    let fixed = TryInto::<[u8; 3]>::try_into(small).unwrap();
    assert_eq!(fixed, [1, 2, 3]);
}