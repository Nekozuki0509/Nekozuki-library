use std::{collections::VecDeque, mem::swap};
use cargo_snippet::snippet;

#[snippet]
fn dfs(pos: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[pos] = true;
    for &i in &g[pos] {
        if !visited[i] {
            dfs(i, g, visited);
        }
    }
}

#[snippet]
fn bfs() {
    let n = 0;
    let g = vec![vec![]];
    let mut dist = vec![!0; n];
    let mut q = VecDeque::new();
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
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { par: (0..n).collect(), siz: vec![1; n] }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }

        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> usize {
        parent = self.root(parent);
        child = self.root(child);
        
        if parent == child {
            return parent;
        }

        if self.siz[parent] < self.siz[child] {
            swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];

        parent
    }

    fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}