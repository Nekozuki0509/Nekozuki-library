use cargo_snippet_more::{snippet_end, snippet_start};

use crate::collection::{Grid, Pos};

snippet_start!(name = "grid", library = "Grid");
pub struct IGrid<T>(Pos, std::marker::PhantomData<T>);
impl<T: proconio::source::Readable<Output = T> + Default + Clone> proconio::source::RuntimeReadable
    for IGrid<T>
{
    type Output = Grid<T>;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
        self,
        source: &mut S,
    ) -> Self::Output {
        let IGrid::<T>(p, _) = self;

        let mut g = Grid::new(p.x, p.y, T::default());
        for p in p {
            g[p] = <T as proconio::source::Readable>::read(source);
        }

        g
    }
}
snippet_end!("grid");

snippet_start!(name = "iset", library = "ISet");
pub struct ISet<T>(usize, std::marker::PhantomData<T>);
impl<T: proconio::source::Readable<Output = T> + Ord> proconio::source::RuntimeReadable
    for ISet<T>
{
    type Output = std::collections::BTreeSet<T>;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
        self,
        source: &mut S,
    ) -> Self::Output {
        let ISet::<T>(n, _) = self;

        let mut s = std::collections::BTreeSet::new();
        for _ in 0..n {
            s.insert(<T as proconio::source::Readable>::read(source));
        }

        s
    }
}
snippet_end!("iset");

snippet_start!(name = "imap", library = "IMap");
pub struct IMap<K, V>(
    usize,
    std::marker::PhantomData<K>,
    std::marker::PhantomData<V>,
);
impl<K: proconio::source::Readable<Output = K> + Ord, V: proconio::source::Readable<Output = V>>
    proconio::source::RuntimeReadable for IMap<K, V>
{
    type Output = std::collections::BTreeMap<K, V>;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
        self,
        source: &mut S,
    ) -> Self::Output {
        let IMap::<K, V>(n, _, _) = self;

        let mut m = std::collections::BTreeMap::new();
        for _ in 0..n {
            m.insert(
                <K as proconio::source::Readable>::read(source),
                <V as proconio::source::Readable>::read(source),
            );
        }

        m
    }
}
snippet_end!("imap");

snippet_start!(name = "ique", library = "IQue");
pub struct IQue<T>(usize, std::marker::PhantomData<T>);
impl<T: proconio::source::Readable<Output = T>> proconio::source::RuntimeReadable for IQue<T> {
    type Output = std::collections::VecDeque<T>;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
        self,
        source: &mut S,
    ) -> Self::Output {
        let IQue::<T>(n, _) = self;

        let mut q = std::collections::VecDeque::new();
        for _ in 0..n {
            q.push_back(<T as proconio::source::Readable>::read(source));
        }

        q
    }
}
snippet_end!("ique");

snippet_start!(name = "iheap", library = "IHeap");
pub struct IHeap<T>(usize, std::marker::PhantomData<T>);
impl<T: proconio::source::Readable<Output = T> + Ord> proconio::source::RuntimeReadable
    for IHeap<T>
{
    type Output = std::collections::BinaryHeap<T>;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
        self,
        source: &mut S,
    ) -> Self::Output {
        let IHeap::<T>(n, _) = self;

        let mut h = std::collections::BinaryHeap::new();
        for _ in 0..n {
            h.push(<T as proconio::source::Readable>::read(source));
        }

        h
    }
}
snippet_end!("iheap");
