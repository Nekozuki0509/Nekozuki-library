use cargo_snippet_more::{snippet_end, snippet_start};

snippet_start!(name = "more_iter", library = "MoreIter");
pub trait MoreIter: Iterator {
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
}

impl<I: Iterator + ?Sized> MoreIter for I {}
snippet_end!("more_iter");
