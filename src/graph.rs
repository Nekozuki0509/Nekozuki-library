use cargo_snippet_more::{p, snippet, snippet_end, snippet_start};

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
    let mut dist = vec![!0; p!(1, n)];
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    dist[0] = 0;
    while let Some(pos) = q.pop_front() {
        for &i in &g[pos] {
            if dist[i] == !0 {
                dist[i] = dist[pos] + 1;
                q.push_back(i);
                p!(0);
            }
        }
    }
    snippet_end!("bfs");
}

snippet_start!(name = "uf", library = "UnionFind");
pub struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

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

snippet_start!(name = "dijkstra", library = "Dijkstra");
const INF_DIJKSTRA: usize = 1 << 31;

pub struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF_DIJKSTRA; n];
        let mut parent = vec![INF_DIJKSTRA; n];
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((std::cmp::Reverse(0), i));
            }
            heap.push((std::cmp::Reverse(INF_DIJKSTRA), i));
        }
        while let Some((std::cmp::Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &(next, cost) in &edge[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((std::cmp::Reverse(distance[next]), next));
                    parent[next] = target;
                }
            }
        }
        Self { distance, parent }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        let mut current = target;
        let mut res = vec![current];
        while self.parent[current] != INF_DIJKSTRA {
            let next = self.parent[current];
            res.push(next);
            current = next;
        }
        res.reverse();
        res
    }
}
snippet_end!("dijkstra");
