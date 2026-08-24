//^
//^ HEAD
//^

//> HEAD -> IMPORTS
use {
    super::Array,
    core::cmp::Ordering
};


//^
//^ EQ
//^

//> EQ -> PARTIAL CONTAINER
const impl<
    Type: [const] PartialEq<Type>, 
    To: [const] AsRef<[Type]>, 
    const N: usize
> PartialEq<To> for Array<Type, N> {
    fn eq(&self, other: &To) -> bool {return self.as_ref().eq(other.as_ref())}
}

//> EQ -> TOTAL
const impl<Type: [const] Eq, const N: usize> Eq for Array<Type, N> {}


//^
//^ CMP
//^

//> CMP -> PARTIAL CONTAINER
const impl<
    Type: [const] PartialOrd<Type>, 
    To: [const] AsRef<[Type]>, 
    const N: usize
> PartialOrd<To> for Array<Type, N> {
    fn partial_cmp(&self, other: &To) -> Option<Ordering> {
        return self.as_ref().partial_cmp(other.as_ref());
    }
}

//> CMP -> TOTAL
const impl<Type: [const] Ord, const N: usize> Ord for Array<Type, N> {
    fn cmp(&self, other: &Self) -> Ordering {return self.as_ref().cmp(other.as_ref())}
}