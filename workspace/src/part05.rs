// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    // We can convert any vector of digits into a number, by removing trailing zeros. The `mut`
    // declaration for `v` here is just like the one in `let mut ...`: We completely own `v`, but Rust
    // still asks us to make our intention of modifying it explicit. This `mut` is *not* part of the
    // type of `from_vec` - the caller has to give up ownership of `v` anyway, so they don't care anymore
    // what you do to it.
    //
    // **Exercise 05.1**: Implement this function.
    //
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        while v.len() > 0 && v[v.len() - 1] == 0 {
            v.pop();
        }

        BigInt { data: v }
    }

    pub fn number_of_digits(&self) -> usize {
        let d = self.data.clone();
        d.len()
    }

    pub fn non_zero_digits(&self) -> u64 {
        let mut c  = 0;
        let data = self.data.clone();

        for d in data {
            if d != 0 {
                c += 1;
            }
        }

        c
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v);
}

// impl Clone for BigInt {
//     fn clone(&self) -> Self {
//         BigInt { data: self.data.clone() }
//     }
// }

// We can also make the type `SomethingOrNothing<T>` implement `Clone`.
use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref v) => Something(v.clone()),
        }
    }
}

use std::fmt;
impl<T: fmt::Display> fmt::Display for SomethingOrNothing<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Something(ref t) => t.fmt(f),
            Nothing          => "Nothing".fmt(f),
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the number of
// digits? The number of non-zero digits? The smallest/largest digit? Of course, these should all just borrow `self`.

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}

