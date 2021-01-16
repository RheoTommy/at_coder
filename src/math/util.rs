use super::num_type::{GCDIdent, MulDivIdent};
use cargo_snippet::snippet;

#[snippet("math_util")]
pub fn pow<T: std::ops::Mul<Output = T> + MulDivIdent + Copy>(x: T, n: u128) -> T {
    if n == 0 {
        T::mul_div_ident()
    } else if n == 1 {
        x
    } else if n % 2 == 1 {
        x * pow(x, n - 1)
    } else {
        pow(x * x, n / 2)
    }
}

#[snippet("math_util")]
pub fn mod_pow<T: std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + MulDivIdent + Copy>(
    x: T,
    n: u128,
    m: T,
) -> T {
    if n == 0 {
        T::mul_div_ident() % m
    } else if n == 1 {
        x % m
    } else if n % 2 == 1 {
        x * mod_pow(x, n - 1, m) % m
    } else {
        mod_pow(x * x % m, n / 2, m)
    }
}

#[snippet("math_util")]
#[snippet("seg_tree")]
pub fn gcd<T: GCDIdent + std::ops::Rem<Output = T> + Copy + std::cmp::Ord>(a: T, b: T) -> T {
    if b == T::gcd_ident() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet("math_util")]
pub fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a < b {
        let (g, y, x) = ext_gcd(b, a);
        return (g, x, y);
    }
    if b == 0 {
        return (a, 1, 0);
    }

    let (g, x, y) = ext_gcd(b, a % b);
    (g, y, x - a / b * y)
}

#[snippet("math_util")]
pub fn lcm<
    T: GCDIdent
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + Copy
        + std::cmp::Ord,
>(
    a: T,
    b: T,
) -> T {
    let g = gcd(a, b);
    a * b / g
}

/// O(√N)
#[snippet("math_util")]
pub fn divisors(n: u128) -> Vec<u128> {
    let mut res = Vec::new();
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res.push(i);
            if n / i != i {
                res.push(n / i);
            }
        }
    }
    res
}

/// O(√N)
#[snippet("math_util")]
pub fn is_prime(n: u128) -> bool {
    for i in 2.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// O(√N)
#[snippet("math_util")]
pub fn primes(mut n: u128) -> std::collections::HashMap<u128, u128> {
    let mut res = std::collections::HashMap::new();
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        if ex != 0 {
            res.insert(i, ex);
        }
    }
    if n != 0 {
        *res.entry(n).or_insert(0) += 1;
    }
    res
}
