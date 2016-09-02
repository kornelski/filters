//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! AND implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct And<T, U> {
    a: T,
    b: U
}

impl<T, U> And<T, U> {

    pub fn new(a: T, b: U) -> And<T, U> {
        And { a: a, b: b }
    }

}

impl_operators!(And, self e { self.a.filter(e) && self.b.filter(e) }, T, U);

pub struct AndVec<I>(Vec<Box<I>>);

impl<I> AndVec<I> {

    pub fn new(i: Vec<Box<I>>) -> AndVec<I> {
        AndVec(i)
    }

    // convenient function to build a AndVec from a Filter using Filter::and_vec()
    //
    // Using this function, one can:
    //
    // ```ignore
    //  some_filter.and_vec()
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

    // convenient function to build a AndVec from a Filter using Filter::and_vec()
    //
    // Using this function, one can:
    //
    // ```ignore
    //  some_filter.and_vec()
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

impl<I, F: Filter<I>> Filter<I> for AndVec<Box<F>> {

    fn filter(&self, e: &I) -> bool {
        self.0.iter().all(|f| f.filter(e))
    }

}

