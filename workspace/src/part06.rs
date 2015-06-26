// ***Remember to enable/add this part in `main.rs`!***

// Rust-101, Part 06: Copy, Lifetimes
// ==================================

// We continue to work on our `BigInt`, so we start by importing what we already established.
use part05::BigInt;

// With `BigInt` being about numbers, we should be able to write a version of `vec_min`
// that computes the minimum of a list of `BigInt`. First, we have to write `min` for `BigInt`.
impl BigInt {
    fn min_try1(self, other: Self) -> Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        // Now our assumption of having no trailing zeros comes in handy:
        // If the lengths of the two numbers differ, we already know which is larger.
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            // **Exercise 06.1**: Fill in this code.
            unimplemented!()
        }
    }
}

// Now we can write `vec_min`. However, in order to make it type-check, we have to make a full (deep) copy of e
// by calling `clone()`.
fn vec_min(v: &Vec<BigInt>) -> Option<BigInt> {
    let mut min: Option<BigInt> = None;
    for e in v {
        unimplemented!()
    }
    min
}

// ## `Copy` types

use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Copy> Copy for SomethingOrNothing<T> {}


// ## Lifetimes

fn head<T>(v: &Vec<T>) -> Option<&T> {
    if v.len() > 0 {
        unimplemented!()
    } else {
        None
    }
}
// Technically, we are returning a pointer to the first element. But doesn't that mean that callers have to be
// careful? Imagine `head` would be a C++ function, and we would write the following code.
/*
  int foo(std::vector<int> v) {
    int *first = head(v);
    v.push_back(42);
    return *first;
  }
*/
fn rust_foo(mut v: Vec<i32>) -> i32 {
    let first: Option<&i32> = head(&v);
    /* v.push(42); */
    *first.unwrap()
}

