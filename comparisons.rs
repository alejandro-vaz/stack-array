//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::Array;

//> HEAD -> CORE
use core::{
    hash::{
        Hash,
        Hasher
    },
    cmp::Ordering
};


//^
//^ COMPARISONS
//^

//> COMPARISONS -> EQ
const impl<
    Type: [const] PartialEq<Type>, 
    To: [const] AsRef<[Type]>, 
    const N: usize
> PartialEq<To> for Array<Type, N> {
    fn eq(&self, other: &To) -> bool {return self.as_ref().eq(other.as_ref())}
}

//> COMPARISONS -> ORD
const impl<
    Type: [const] PartialOrd<Type>, 
    To: [const] AsRef<[Type]>, 
    const N: usize
> PartialOrd<To> for Array<Type, N> {
    fn partial_cmp(&self, other: &To) -> Option<Ordering> {
        return self.as_ref().partial_cmp(other.as_ref());
    }
}

//> COMPARISONS -> HASH
impl<Type: Hash, const N: usize> Hash for Array<Type, N> {
    fn hash<Hashing: Hasher>(&self, state: &mut Hashing) {
        return Hash::hash(self.as_ref(), state);
    }
}

//> COMPARISONS -> TOTAL EQ
const impl<Type: [const] Eq, const N: usize> Eq for Array<Type, N> {}

//> COMPARISONS -> TOTAL ORD
const impl<Type: [const] Ord, const N: usize> Ord for Array<Type, N> {
    fn cmp(&self, other: &Self) -> Ordering {return self.as_ref().cmp(other.as_ref())}
}