//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! OR implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Or<T, U> {
    a: T,
    b: U
}

impl<T, U> Or<T, U> {

    pub fn new(a: T, b: U) -> Or<T, U> {
        Or { a: a, b: b }
    }

}

impl_operators!(Or, self e { self.a.filter(e) || self.b.filter(e) }, T, U);

pub struct OrVec<I>(Vec<Box<I>>);

impl<I> OrVec<I> {

    pub fn new(i: Vec<Box<I>>) -> OrVec<I> {
        OrVec(i)
    }

    // convenient function to build a OrVec from a Filter using Filter::or_vec()
    //
    // Using this function, one can:
    //
    // ```ignore
    //  some_filter.or_vec()
    //      .push(another)
    //      .push(filter)
    //      .push(which)
    //      .push(filters)
    //      .and(something_else)
    //      .filter(&element)
    //  ```
    pub fn push(&mut self, i: I) -> &mut Self {
        self.0.push(Box::new(i));
        self
    }

}

impl<I, F: Filter<I>> Filter<I> for OrVec<Box<F>> {

    fn filter(&self, e: &I) -> bool {
        self.0.iter().any(|f| f.filter(e))
    }

}

