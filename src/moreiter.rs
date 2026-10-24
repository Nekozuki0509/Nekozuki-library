use cargo_snippet_more::{snippet_end, snippet_start};

snippet_start!(name = "more_iter", library = "MoreIter");
pub trait MoreIter: Iterator {
    fn to_digit(self) -> impl Iterator<Item = usize>
    where
        Self: Sized + Iterator<Item = char>,
    {
        self.map(|c| c.to_digit(10).unwrap() as usize)
    }

    fn sum(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: std::iter::Sum,
    {
        Iterator::sum(self)
    }

    fn join_space(mut self) -> String
    where
        Self: Sized,
        Self::Item: std::fmt::Display,
    {
        itertools::Itertools::join(&mut self, " ")
    }

    fn join_line(mut self) -> String
    where
        Self: Sized,
        Self::Item: std::fmt::Display,
    {
        itertools::Itertools::join(&mut self, "\n")
    }

    fn print_space(self)
    where
        Self: Sized,
        Self::Item: std::fmt::Display,
    {
        println!("{}", self.join_space());
    }

    fn print_line(self)
    where
        Self: Sized,
        Self::Item: std::fmt::Display,
    {
        println!("{}", self.join_line());
    }

    fn left_sorted<T: Ord, U>(self) -> impl Iterator
    where
        Self: Sized + Iterator<Item = (T, U)>,
    {
        itertools::Itertools::sorted_by(self, |i, j| i.0.cmp(&j.0))
    }

    fn right_sorted<T, U: Ord>(self) -> impl Iterator
    where
        Self: Sized + Iterator<Item = (T, U)>,
    {
        itertools::Itertools::sorted_by(self, |i, j| i.1.cmp(&j.1))
    }

    fn left<T, U>(self) -> impl Iterator
    where
        Self: Sized + Iterator<Item = (T, U)>,
    {
        self.map(|(l, _)| l)
    }

    fn right<T, U>(self) -> impl Iterator
    where
        Self: Sized + Iterator<Item = (T, U)>,
    {
        self.map(|(_, r)| r)
    }

    fn rle(self) -> impl Iterator
    where
        Self: Sized,
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

    fn filter_v<T: PartialEq>(self, v: &T) -> impl Iterator
    where
        Self: Sized + Iterator<Item = T>,
    {
        self.filter(move |x| x == v)
    }

    fn filter_not_v<T: PartialEq>(self, v: &T) -> impl Iterator
    where
        Self: Sized + Iterator<Item = T>,
    {
        self.filter(move |x| x != v)
    }
}

impl<I: Iterator + ?Sized> MoreIter for I {}
snippet_end!("more_iter");
