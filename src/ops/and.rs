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

}

impl<I, F: Filter<I>> Filter<I> for AndVec<F> {

    fn filter(&self, e: &I) -> bool {
        self.0.iter().all(|f| f.filter(e))
    }

}

