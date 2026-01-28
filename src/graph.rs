use cargo_snippet_more::{snippet, snippet_end, snippet_start};

#[snippet]
pub fn dfs(pos: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[pos] = true;
    for &i in &g[pos] {
        if !visited[i] {
            dfs(i, g, visited);
        }
    }
}

pub fn bfs() {
    let n = 0;
    let g = vec![vec![]];
    snippet_start!("bfs");
    let mut dist = vec![!0; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    dist[0] = 0;
    while let Some(pos) = q.pop_front() {
        for &i in &g[pos] {
            if dist[i] == -1 {
                dist[i] = dist[pos] + 1;
                q.push_back(i);
            }
        }
    }
    snippet_end!("bfs");
}

snippet_start!(name = "uf", library = "UnionFind");
//#[snippet("uf")]
pub struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

//#[snippet("uf")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }

        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    pub fn unite(&mut self, mut parent: usize, mut child: usize) -> usize {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return parent;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];

        parent
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
snippet_end!("uf");
