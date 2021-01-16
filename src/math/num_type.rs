#![cargo_snippet::snippet("math_util")]
#![cargo_snippet::snippet("mod_int")]
#![cargo_snippet::snippet("lis")]
#![cargo_snippet::snippet("mod_int")]

/// 加減算
pub trait AddSubIdent {
    fn add_sub_ident() -> Self;
}

/// 乗除算
pub trait MulDivIdent {
    fn mul_div_ident() -> Self;
}

/// GCD
pub trait GCDIdent {
    fn gcd_ident() -> Self;
}

/// LCM
pub trait LCMIdent {
    fn lcm_ident() -> Self;
}

/// Max
pub trait MaxIdent {
    fn max_ident() -> Self;
}

/// Min
pub trait MinIdent {
    fn min_ident() -> Self;
}

/// Xor
pub trait XorIdent {
    fn xor_ident() -> Self;
}

/// 変換用
pub struct Num(pub i128);

macro_rules! impl_num_from {
    ($t:ty) => {
        impl From<Num> for $t {
            fn from(t: Num) -> $t {
                t.0 as $t
            }
        }
    };
}
impl_num_from!(i8);
impl_num_from!(i16);
impl_num_from!(i32);
impl_num_from!(i64);
impl_num_from!(i128);
impl_num_from!(isize);
impl_num_from!(u8);
impl_num_from!(u16);
impl_num_from!(u32);
impl_num_from!(u64);
impl_num_from!(u128);
impl_num_from!(usize);

macro_rules! impl_num_into {
    ($t:ty) => {
        impl Into<Num> for $t {
            fn into(self) -> Num {
                Num(self as i128)
            }
        }
    };
}
impl_num_into!(i8);
impl_num_into!(i16);
impl_num_into!(i32);
impl_num_into!(i64);
impl_num_into!(i128);
impl_num_into!(isize);
impl_num_into!(u8);
impl_num_into!(u16);
impl_num_into!(u32);
impl_num_into!(u64);
impl_num_into!(u128);
impl_num_into!(usize);

impl<T: From<Num>> AddSubIdent for T {
    fn add_sub_ident() -> Self {
        Num(0).into()
    }
}

impl<T: From<Num>> MulDivIdent for T {
    fn mul_div_ident() -> Self {
        Num(1).into()
    }
}

impl<T: From<Num>> GCDIdent for T {
    fn gcd_ident() -> Self {
        Num(0).into()
    }
}

impl<T: From<Num>> LCMIdent for T {
    fn lcm_ident() -> Self {
        Num(1).into()
    }
}

macro_rules! impl_min {
    ($t:ident) => {
        impl MinIdent for $t {
            fn min_ident() -> Self {
                std::$t::MIN
            }
        }
    };
}
impl_min!(i8);
impl_min!(i16);
impl_min!(i32);
impl_min!(i64);
impl_min!(i128);
impl_min!(isize);
impl_min!(u8);
impl_min!(u16);
impl_min!(u32);
impl_min!(u64);
impl_min!(u128);
impl_min!(usize);

macro_rules! impl_max {
    ($t:ident) => {
        impl MaxIdent for $t {
            fn max_ident() -> Self {
                std::$t::MAX
            }
        }
    };
}
impl_max!(i8);
impl_max!(i16);
impl_max!(i32);
impl_max!(i64);
impl_max!(i128);
impl_max!(isize);
impl_max!(u8);
impl_max!(u16);
impl_max!(u32);
impl_max!(u64);
impl_max!(u128);
impl_max!(usize);

impl<T: From<Num>> XorIdent for T {
    fn xor_ident() -> Self {
        Num(0).into()
    }
}
