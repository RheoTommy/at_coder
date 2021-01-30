use cargo_snippet::snippet;

/// UnionFind構造体
#[snippet("union_find")]
pub struct UnionFindTree {
    /// 頂点`i`の親を格納する配列
    parents: Vec<usize>,
    /// 頂点`i`が親であるときのその木の頂点数
    sizes: Vec<usize>,
    /// 重み付きUnionFindを使う際の重みの格納配列
    weights: Vec<isize>,
    /// 頂点`i`が属する木がループを持っているかどうか
    has_loops: Vec<bool>,
}

#[snippet("union_find")]
impl UnionFindTree {
    /// UnionFind初期化
    /// 計算量はO(n)
    pub fn new(n: usize) -> Self {
        let parents = (0..n).collect();
        let sizes = vec![1; n];
        let weights = vec![0; n];
        let has_loops = vec![false; n];
        UnionFindTree {
            parents,
            sizes,
            weights,
            has_loops,
        }
    }

    /// 親を再帰的に求め、途中の計算結果をもとに親の書き換えを行う関数
    /// 計算量はO(a(n)))
    pub fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            let tmp = self.root(self.parents[x]);
            self.weights[x] += self.weights[self.parents[x]];
            self.parents[x] = tmp;
            tmp
        }
    }

    pub fn size(&mut self, x: usize) -> usize {
        let y = self.root(x);
        self.sizes[y]
    }

    pub fn has_loop(&mut self, x: usize) -> bool {
        let y = self.root(x);
        self.has_loops[y]
    }

    /// 2つの頂点が同じ木に属しているかの判定
    /// `self.root()`を呼び出すため、`&mut self`を引数に取る。そのため、命名に`is_`を使っていない
    /// 計算量はO(a(n))
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// 重み付きUnionFindを考える際のUnite関数
    /// 計算量はO(a(n))
    pub fn unite_with_weight(&mut self, x: usize, y: usize, w: isize) {
        let root_x = self.root(x);
        let root_y = self.root(y);

        if self.same(x, y) {
            self.has_loops[root_x] = true;
            self.has_loops[root_y] = true;
        } else if self.sizes[root_x] >= self.sizes[root_y] {
            self.parents[root_y] = root_x;
            self.has_loops[root_x] |= self.has_loops[root_y];
            self.sizes[root_x] += self.sizes[root_y];
            self.weights[root_y] = -w - self.weights[y] + self.weights[x];
        } else {
            self.parents[root_x] = root_y;
            self.has_loops[root_y] |= self.has_loops[root_x];
            self.sizes[root_y] += self.sizes[root_x];
            self.weights[root_x] = w + self.weights[y] - self.weights[x];
        }
    }

    /// 重みを考慮しない際のUnite関数
    /// 重みとして0を与えているだけであり、計算量は同じくO(a(n))
    pub fn unite(&mut self, x: usize, y: usize) {
        self.unite_with_weight(x, y, 0);
    }

    /// 重み付きUnionFindにおいて、2つの頂点の距離を返す関数
    /// 2つの頂点が同じ木に属していない場合は`None`を返す
    pub fn diff(&mut self, x: usize, y: usize) -> Option<isize> {
        if self.same(x, y) {
            Some(self.weights[x] - self.weights[y])
        } else {
            None
        }
    }

    pub fn is_parent(&self, x: usize) -> bool {
        self.parents[x] == x
    }
}
