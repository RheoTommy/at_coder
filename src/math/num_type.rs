use cargo_snippet::snippet;

/// 加減算
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub trait AddSubIdent {
    fn add_sub_ident() -> Self;
}

/// 乗除算
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub trait MulDivIdent {
    fn mul_div_ident() -> Self;
}

/// GCD
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub trait GCDIdent {
    fn gcd_ident() -> Self;
}

/// LCM
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub trait LCMIdent {
    fn lcm_ident() -> Self;
}

/// Max
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub trait MaxIdent {
    fn max_ident() -> Self;
}

/// Min
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub trait MinIdent {
    fn min_ident() -> Self;
}

/// Xor
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub trait XorIdent {
    fn xor_ident() -> Self;
}

/// 変換用
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
pub struct Num(pub i64);

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
macro_rules! impl_num_from {
    ($($t:ty),*) => {$(
        impl From<Num> for $t {
            fn from(t: Num) -> $t {
                t.0 as $t
            }
        }
    )*};
}
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl_num_from!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
macro_rules! impl_num_into {
    ($($t:ty),*) => {$(
        impl Into<Num> for $t {
            fn into(self) -> Num {
                Num(self as i64)
            }
        }
    )*};
}
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl_num_into!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl<T: From<Num>> AddSubIdent for T {
    fn add_sub_ident() -> Self {
        Num(0).into()
    }
}

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl<T: From<Num>> MulDivIdent for T {
    fn mul_div_ident() -> Self {
        Num(1).into()
    }
}

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl<T: From<Num>> GCDIdent for T {
    fn gcd_ident() -> Self {
        Num(0).into()
    }
}

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl<T: From<Num>> LCMIdent for T {
    fn lcm_ident() -> Self {
        Num(1).into()
    }
}

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
macro_rules! impl_min {
    ($($t:ident),*) => {$(
        impl MinIdent for $t {
            fn min_ident() -> Self {
                std::$t::MIN
            }
        }
    )*};
}
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl_min!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
macro_rules! impl_max {
    ($($t:ident),*) => {$(
        impl MaxIdent for $t {
            fn max_ident() -> Self {
                std::$t::MAX
            }
        }
    )*};
}
#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl_max!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize);

#[snippet("math_util")]
#[snippet("mod_int")]
#[snippet("lis")]
impl<T: From<Num>> XorIdent for T {
    fn xor_ident() -> Self {
        Num(0).into()
    }
}
