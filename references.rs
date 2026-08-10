//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::{
    slice::{
        from_raw_parts as fat,
        from_raw_parts_mut as mutfat
    },
    borrow::{
        Borrow,
        BorrowMut
    },
    ops::{
        Deref,
        DerefMut
    }
};


//^
//^ DEREF
//^

//> DEREF -> CONSTANT
const impl<Type, const N: usize> Deref for Array<Type, N> {
    type Target = [Type];
    fn deref(&self) -> &Self::Target {
        return unsafe {fat(self.data.as_ptr().cast(), self.length)};
    }
}

//> DEREF -> MUTABLE
const impl<Type, const N: usize> DerefMut for Array<Type, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return unsafe {mutfat(self.data.as_mut_ptr().cast(), self.length)};
    }
}


//^
//^ REFERENCES
//^

//> REFERENCES -> SLICE
const impl<Type, const N: usize> AsRef<[Type]> for Array<Type, N> {
    fn as_ref(&self) -> &[Type] {return self.deref()}
}

//> REFERENCES -> MUTABLE SLICE
const impl<Type, const N: usize> AsMut<[Type]> for Array<Type, N> {
    fn as_mut(&mut self) -> &mut [Type] {return self.deref_mut()}
}


//^
//^ BORROW
//^

//> BORROW -> CONSTANT
const impl<Type, const N: usize> Borrow<[Type]> for Array<Type, N> {
    fn borrow(&self) -> &[Type] {self.as_ref()}
}

//> BORROW -> MUTABLE
const impl<Type, const N: usize> BorrowMut<[Type]> for Array<Type, N> {
    fn borrow_mut(&mut self) -> &mut [Type] {self.as_mut()}
}