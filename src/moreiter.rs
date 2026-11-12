use crate::collection::{Pair, PairExt, PairRef, PairRefExt};
use cargo_snippet_more::{snippet_end, snippet_start};
snippet_start!(name = "more_iter", library = "MoreIter", include = "pair");
macro_rules! iter_arith_left {
    ($name:ident, $Tr:ident) => {
        fn $name<V: Clone>(
            self,
            v: V,
        ) -> impl Iterator<
            Item = (
                <<Self::Item as Pair>::Left as ::std::ops::$Tr<V>>::Output,
                <Self::Item as Pair>::Right,
            ),
        >
        where
            Self::Item: Pair,
            <Self::Item as Pair>::Left: ::std::ops::$Tr<V>,
        {
            self.map(move |p| p.$name(v.clone()))
        }
    };
}
macro_rules! iter_arith_right {
    ($name:ident, $Tr:ident) => {
        fn $name<V: Clone>(
            self,
            v: V,
        ) -> impl Iterator<
            Item = (
                <Self::Item as Pair>::Left,
                <<Self::Item as Pair>::Right as ::std::ops::$Tr<V>>::Output,
            ),
        >
        where
            Self::Item: Pair,
            <Self::Item as Pair>::Right: ::std::ops::$Tr<V>,
        {
            self.map(move |p| p.$name(v.clone()))
        }
    };
}
macro_rules! iter_side {
    ($V:ident, $get:ident, $cmp:ident,
     $filter:ident, $filter_not:ident, $filter_by:ident, $filter_not_by:ident,
     $sorted:ident, $sorted_by:ident, $sorted_by_key:ident) => {
        fn $filter(self, v: <Self::Item as PairRef>::$V) -> impl Iterator<Item = Self::Item>
        where
            Self::Item: PairRef,
            <Self::Item as PairRef>::$V: PartialEq,
        {
            self.filter(move |i| *i.$get() == v)
        }
        fn $filter_not(self, v: <Self::Item as PairRef>::$V) -> impl Iterator<Item = Self::Item>
        where
            Self::Item: PairRef,
            <Self::Item as PairRef>::$V: PartialEq,
        {
            self.filter(move |i| *i.$get() != v)
        }
        fn $filter_by<F>(self, mut f: F) -> impl Iterator<Item = Self::Item>
        where
            Self::Item: PairRef,
            F: FnMut(&<Self::Item as PairRef>::$V) -> bool,
        {
            self.filter(move |i| f(i.$get()))
        }
        fn $filter_not_by<F>(self, mut f: F) -> impl Iterator<Item = Self::Item>
        where
            Self::Item: PairRef,
            F: FnMut(&<Self::Item as PairRef>::$V) -> bool,
        {
            self.filter(move |i| !f(i.$get()))
        }
        fn $sorted(self) -> ::std::vec::IntoIter<Self::Item>
        where
            Self::Item: PairRef,
            <Self::Item as PairRef>::$V: Ord,
        {
            ::itertools::Itertools::sorted_by(self, |a, b| a.$cmp(b))
        }
        fn $sorted_by<F>(self, mut f: F) -> ::std::vec::IntoIter<Self::Item>
        where
            Self::Item: PairRef,
            F: FnMut(
                &<Self::Item as PairRef>::$V,
                &<Self::Item as PairRef>::$V,
            ) -> ::std::cmp::Ordering,
        {
            ::itertools::Itertools::sorted_by(self, |a, b| f(a.$get(), b.$get()))
        }
        fn $sorted_by_key<K: Ord, F>(self, mut f: F) -> ::std::vec::IntoIter<Self::Item>
        where
            Self::Item: PairRef,
            F: FnMut(&<Self::Item as PairRef>::$V) -> K,
        {
            ::itertools::Itertools::sorted_by_key(self, |i| f(i.$get()))
        }
    };
}
pub trait MoreIter: Iterator + Sized {
    fn to_digit(self) -> impl Iterator<Item = usize>
    where
        Self: Iterator<Item = char>,
    {
        self.map(|c| c.to_digit(10).unwrap() as usize)
    }
    fn sume(self) -> Self::Item
    where
        Self::Item: std::iter::Sum,
    {
        Iterator::sum(self)
    }
    fn join_space(mut self) -> String
    where
        Self::Item: std::fmt::Display,
    {
        itertools::Itertools::join(&mut self, " ")
    }
    fn join_line(mut self) -> String
    where
        Self::Item: std::fmt::Display,
    {
        itertools::Itertools::join(&mut self, "\n")
    }
    fn print_space(self)
    where
        Self::Item: std::fmt::Display,
    {
        println!("{}", self.join_space());
    }
    fn print_line(self)
    where
        Self::Item: std::fmt::Display,
    {
        println!("{}", self.join_line());
    }
    fn rle(self) -> std::collections::vec_deque::IntoIter<(Self::Item, usize)>
    where
        Self::Item: PartialEq,
    {
        self.fold(std::collections::VecDeque::new(), |mut acc, x| {
            if let Some((l, v)) = acc.pop_back() {
                if l == x {
                    acc.push_back((l, v + 1));
                } else {
                    acc.push_back((l, v));
                    acc.push_back((x, 1));
                }
            } else {
                acc.push_back((x, 1));
            }
            acc
        })
        .into_iter()
    }
    fn filter_v<T: PartialEq>(self, v: &T) -> impl Iterator<Item = T>
    where
        Self: Iterator<Item = T>,
    {
        self.filter(move |x| x == v)
    }
    fn filter_not_v<T: PartialEq>(self, v: &T) -> impl Iterator<Item = T>
    where
        Self: Iterator<Item = T>,
    {
        self.filter(move |x| x != v)
    }
    fn arg_sort<T: num::Integer + Copy>(self) -> impl Iterator<Item = (T, T)>
    where
        Self: Iterator<Item = (T, T)>,
    {
        itertools::Itertools::sorted_by(self, |&(x0, y0), &(x1, y1)| {
            ((y0, x0) < (num::zero(), num::zero()))
                .cmp(&((y1, x1) < (num::zero(), num::zero())))
                .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
        })
    }
    fn left(self) -> impl Iterator<Item = <Self::Item as Pair>::Left>
    where
        Self::Item: Pair,
    {
        self.map(Pair::left)
    }
    fn right(self) -> impl Iterator<Item = <Self::Item as Pair>::Right>
    where
        Self::Item: Pair,
    {
        self.map(Pair::right)
    }
    iter_arith_left!(left_add, Add);
    iter_arith_left!(left_sub, Sub);
    iter_arith_left!(left_mul, Mul);
    iter_arith_left!(left_div, Div);
    iter_arith_right!(right_add, Add);
    iter_arith_right!(right_sub, Sub);
    iter_arith_right!(right_mul, Mul);
    iter_arith_right!(right_div, Div);
    iter_side!(
        LeftValue,
        left_ref,
        left_cmp,
        left_filter,
        left_filter_not,
        left_filter_by,
        left_filter_not_by,
        left_sorted,
        left_sorted_by,
        left_sorted_by_key
    );
    iter_side!(
        RightValue,
        right_ref,
        right_cmp,
        right_filter,
        right_filter_not,
        right_filter_by,
        right_filter_not_by,
        right_sorted,
        right_sorted_by,
        right_sorted_by_key
    );
}
impl<I: Iterator> MoreIter for I {}
snippet_end!("more_iter");
