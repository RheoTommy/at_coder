pub fn pow<T: std::ops::Mul<Output=T> + From<isize> + Copy>(x: T, n: usize) -> T {
    if n == 0 {
        1isize.into()
    } else if n == 1 {
        x
    } else if n % 2 == 1 {
        x * pow(x, n - 1)
    } else {
        pow(x * x, n / 2)
    }
}

pub fn gcd<T: From<usize> + std::ops::Rem<Output=T> + Copy + std::cmp::Ord>(a: T, b: T) -> T {
    if a < b {
        gcd(b, a)
    } else if b == 0.into() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn ext_gcd(a: isize, b: isize) -> (isize, isize, isize) {
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

pub fn lcm<
    T: From<usize>
    + std::ops::Mul<Output=T>
    + std::ops::Div<Output=T>
    + std::ops::Rem<Output=T>
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
pub fn divisors(n: usize) -> Vec<usize> {
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
pub fn is_prime(n: usize) -> bool {
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
pub fn primes(mut n: usize) -> std::collections::HashMap<usize, usize> {
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