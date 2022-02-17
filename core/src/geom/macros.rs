macro_rules! forward_binop_generic {
    ($trait:ident :: $fn:ident for $name:ident) => {
        impl<'a, T: Scalar + ~const $trait<Output=T>> const $trait<&'a $name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn $fn(self, rhs: &'a $name<T>) -> Self::Output {
                $trait::$fn(self, *rhs)
            }
        }
        impl<'a, T: Scalar + ~const $trait<Output=T>> const $trait<$name<T>> for &'a $name<T> {
            type Output = $name<T>;

            #[inline]
            fn $fn(self, rhs: $name<T>) -> Self::Output {
                $trait::$fn(*self, rhs)
            }
        }
        impl<'a, T: Scalar + ~const $trait<Output=T>> const $trait<&'a $name<T>> for &'a $name<T> {
            type Output = $name<T>;

            #[inline]
            fn $fn(self, rhs: &'a $name<T>) -> Self::Output {
                $trait::$fn(*self, *rhs)
            }
        }
    }
}

pub(crate) use forward_binop_generic;