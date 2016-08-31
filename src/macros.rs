#[macro_export]
macro_rules! construct_filter {

    ( $a:ident $( and $e:ident )+ ) => {{
        use $crate::ops::and::And;
        And::new($a, construct_filter!( $( $e and )* ))
    }};
    ( $a:ident and ) => {{ $a }};


}

#[cfg(test)]
mod tests {
    use filter::*;

    #[test]
    fn test_simple_construction_and() {
        let a = |&a: &usize| a > 0;
        let b = |&a: &usize| a < 5;

        let c = construct_filter! { a and b };

        assert!(!c.filter(&0));
        assert!(c.filter(&1));
        assert!(c.filter(&2));
        assert!(c.filter(&3));
        assert!(c.filter(&4));
        assert!(!c.filter(&5));

        for n in 6..42 {
            assert!(!c.filter(&n));
        }
    }


}
