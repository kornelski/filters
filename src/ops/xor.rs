//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! XOR implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct XOr<T, U> {
    a: T,
    b: U
}

impl<T, U> XOr<T, U> {

    pub fn new(a: T, b: U) -> XOr<T, U> {
        XOr { a: a, b: b }
    }

}

impl_operators!(XOr, self e { self.a.filter(e) ^ self.b.filter(e) }, T, U);

pub struct XOrVec<I>(Vec<Box<I>>);

impl<I> XOrVec<I> {

    pub fn new(i: Vec<Box<I>>) -> XOrVec<I> {
        XOrVec(i)
    }

    // convenient function to build a XOrVec from a Filter using Filter::xor_vec()
    //
    // Using this function, one can:
    //
    // ```ignore
    //  some_filter.xor_vec()
    //      .push(another)
    //      .push(filter)
    //      .push(which)
    //      .push(filters)
    //      .or(something_else)
    //      .filter(&element)
    //  ```
    pub fn push(&mut self, i: I) -> &mut Self {
        self.0.push(Box::new(i));
        self
    }

}

impl<I, F: Filter<I>> Filter<I> for XOrVec<Box<F>> {

    fn filter(&self, e: &I) -> bool {
        let mut b = false;
        for f in self.0.iter() {
            if f.filter(e) {
                if b { // if there is already a filter which returned true, xor yields false
                    return false;
                } else { // if there is no filter yet that yielded true, we can toggle the flag
                    b = true;
                }
            }
        }
        return b
    }

}

