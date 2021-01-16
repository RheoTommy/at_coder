use cargo_snippet::snippet;

/// 構築O(N log log N)で素因数分解系の操作が高速に行える
#[snippet("eratosthenes")]
pub struct Eratosthenes {
    /// n以下の整数iを割り切る最小の素数
    min_prime: Vec<usize>,
}

#[snippet("eratosthenes")]
impl Eratosthenes {
    /// 構築O(N log log N)
    pub fn new(n: usize) -> Self {
        let mut min_prime = (0..=n).collect::<Vec<_>>();
        let mut i = 2usize;
        while i * i <= n {
            if min_prime[i] == i {
                let mut j = 1;
                while i * j <= n {
                    if min_prime[i * j] == i * j {
                        min_prime[i * j] = i;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        Self { min_prime }
    }

    /// 素数判定O(1)
    pub fn is_prime(&self, i: usize) -> bool {
        if i <= 1 {
            return false;
        }
        self.min_prime[i] == i
    }

    /// 素因数分解O(log N)
    pub fn factorization(&self, i: usize) -> std::collections::HashMap<usize, usize> {
        assert_ne!(i, 0);
        let mut map = std::collections::HashMap::new();
        let mut now = i;
        while now != 1 {
            let p = self.min_prime[now];
            *map.entry(p).or_insert(0) += 1;
            now /= p;
        }
        map
    }

    /// 最大公約数O((log N)^2)
    pub fn gcd(&self, i: usize, j: usize) -> std::collections::HashMap<usize, usize> {
        assert_ne!(i, 0);
        assert_ne!(j, 0);
        let mut map = std::collections::HashMap::new();
        let im = self.factorization(i);
        let jm = self.factorization(j);
        for (k, v) in im {
            if jm.contains_key(&k) {
                map.insert(k, v.min(jm[&k]));
            }
        }
        map
    }

    /// 最小公倍数O((log N)^2)
    pub fn lcm(&self, i: usize, j: usize) -> std::collections::HashMap<usize, usize> {
        assert_ne!(i, 0);
        assert_ne!(j, 0);
        let mut map = std::collections::HashMap::new();
        let im = self.factorization(i);
        let jm = self.factorization(j);
        for (&k, &v) in &im {
            if jm.contains_key(&k) {
                map.insert(k, v.max(jm[&k]));
            } else {
                map.insert(k, v);
            }
        }
        for (k, v) in jm {
            if !im.contains_key(&k) {
                map.insert(k, v);
            }
        }
        map
    }
}
