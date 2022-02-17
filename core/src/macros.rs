pub(crate) macro replace_expr ($tt:tt => $sub:expr) {
$sub
}

pub(crate) macro forward_binop ($op:ident :: $fn:ident for $name:ident, $($field:ident),+) {
impl<'a, T> const $op<&'a $name<T>> for $name<T> where T: ~const Drop + ~const $op<&'a T, Output=T> {
        type Output = $name<T>;

        #[inline]
        fn $fn(self, rhs: &'a $name<T>) -> Self::Output {
            Self {
                $(
                    $field: $op::$fn(self.$field, &rhs.$field)
                ),+
            }
        }
    }
    impl<'a, T> const $op<$name<T>> for &'a $name<T> where T: ~const Drop, &'a T: ~const $op<T, Output=T> {
        type Output = $name<T>;

        #[inline]
        fn $fn(self, rhs: $name<T>) -> Self::Output {
            $name {
                $(
                    $field: $op::$fn(&self.$field, rhs.$field)
                ),+
            }
        }
    }
    impl<'a, T> const $op<&'a $name<T>> for &'a $name<T> where &'a T: ~const $op<&'a T, Output=T> {
        type Output = $name<T>;

        #[inline]
        fn $fn(self, rhs: &'a $name<T>) -> Self::Output {
            $name {
                $(
                    $field: $op::$fn(&self.$field, &rhs.$field)
                ),+
            }
        }
    }
}

pub(crate) macro strip_plus(+ $($rest:tt)*) {
    $($rest)*
}