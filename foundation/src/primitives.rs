macro_rules! param {
    ($name:ident, $type:ty, $value:expr) => {
        pub struct $name;
        impl crate::traits::Get<$type> for $name {
            fn get() -> $type {
                $value.into()
            }
        }
    };
}

macro_rules! const_getter {
    ($name:ident, $type:ty) => {
        pub struct $name<const T: $type>;
        impl<const T: $type> crate::traits::Get<$type> for $name<T> {
            fn get() -> $type {
                T
            }
        }
    };
}

const_getter!(ConstI8, i8);
const_getter!(ConstI16, i16);
const_getter!(ConstI32, i32);
const_getter!(ConstI64, i64);
const_getter!(ConstI128, i128);
const_getter!(ConstIsize, isize);

const_getter!(ConstU8, u8);
const_getter!(ConstU16, u16);
const_getter!(ConstU32, u32);
const_getter!(ConstU64, u64);
const_getter!(ConstU128, u128);
const_getter!(ConstUsize, usize);

const_getter!(ConstBool, bool);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Get;

    #[test]
    fn const_getters_works() {
        assert_eq!(ConstI8::<42>::get(), 42);
        assert_eq!(ConstI16::<42>::get(), 42);
        assert_eq!(ConstI32::<42>::get(), 42);
        assert_eq!(ConstI64::<42>::get(), 42);
        assert_eq!(ConstI128::<42>::get(), 42);
        assert_eq!(ConstIsize::<42>::get(), 42);

        assert_eq!(ConstU8::<42>::get(), 42);
        assert_eq!(ConstU16::<42>::get(), 42);
        assert_eq!(ConstU32::<42>::get(), 42);
        assert_eq!(ConstU64::<42>::get(), 42);
        assert_eq!(ConstU128::<42>::get(), 42);
        assert_eq!(ConstUsize::<42>::get(), 42);

        assert_eq!(ConstBool::<true>::get(), true);
    }

    #[test]
    fn param_getter_works() {
        param!(S, String, "Hello");
        assert_eq!(S::get(), "Hello".to_string());
    }
}
