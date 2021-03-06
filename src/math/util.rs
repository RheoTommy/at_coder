use super::num_type::{GCDIdent, MulDivIdent};
use cargo_snippet::snippet;

#[snippet("math_util")]
pub fn pow<T: std::ops::Mul<Output = T> + MulDivIdent + Copy>(x: T, n: u64) -> T {
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
    n: u64,
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
pub fn gcd<T: GCDIdent + std::ops::Rem<Output = T> + Copy + std::cmp::Ord>(
    a: T,
    b: T,
) -> T {
    if b == T::gcd_ident() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet("math_util")]
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        if a < 0 {
            return (-a, -1, 0);
        } else {
            return (a, 1, 0);
        }
    }
    let (g, s, t) = ext_gcd(b, a % b);
    return (g, t, s - (a / b) * t);
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
pub fn divisors(n: u64) -> Vec<u64> {
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
pub fn is_prime(n: u64) -> bool {
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
pub fn primes(mut n: u64) -> std::collections::BTreeMap<u64, u64> {
    let mut res = std::collections::BTreeMap::new();
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
//
#[snippet("math_util")]
pub fn float_to_int(s: &[char], x: u32) -> i64 {
    if !s.contains(&'.') {
        return s.iter().collect::<String>().parse::<i64>().unwrap() * 10i64.pow(x);
    }

    let n = s.len();
    let i = s
        .iter()
        .enumerate()
        .filter(|(_, ci)| **ci == '.')
        .next()
        .unwrap()
        .0;
    let l = n - i - 1;
    let t = s
        .iter()
        .skip_while(|ci| **ci == '0')
        .filter(|ci| **ci != '.')
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
        * 10i64.pow(x - l as u32);

    t
}
