//^
//^ ERRORS
//^

//> ERRORS -> UNMATCHED CAPACITY
#[derive(Debug)]
pub struct UnmatchedCapacity<const EXPECTED: usize> {
    pub present: usize
}

//> ERRORS -> CAPACITY EXCEEDED
#[derive(Debug)]
pub struct CapacityExceeded<const N: usize> {
    pub expected: usize
}