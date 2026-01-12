#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet = "pos"]
#[proconio::derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Pos {
    x: usize,
    y: usize,
}

#[snippet = "pos"]
impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn new_if_is_valid(&self, d: UnsafePos, n: usize) -> Option<Self> {
        let next = UnsafePos::new(self.x as isize + d.x, self.y as isize + d.y);
        if next.is_valid(n as isize) {
            Some(Self::new(next.x as usize, next.y as usize))
        } else {
            None
        }
    }
}

#[snippet = "unsafe_pos"]
#[derive(Debug, Clone, Copy, PartialEq)]
struct UnsafePos {
    x: isize,
    y: isize,
}

#[snippet = "unsafe_pos"]
impl UnsafePos {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_valid(&self, n: isize) -> bool {
        0 <= self.x && self.x < n && 0 <= self.y && self.y < n
    }
}

#[snippet = "grid"]
struct Grid<T> {
    g: Vec<Vec<T>>,
}

#[snippet = "grid"]
impl<T: Clone> Grid<T> {
    fn new(n: usize) -> Self {
        Self { g: vec![vec![]; n] }
    }

    fn new_with_default(n: usize, d: T) -> Self {
        Self {
            g: vec![vec![d; n]; n],
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.g.iter()
    }

    fn pos_iter(&self) -> impl Iterator<Item = (Pos, &T)> {
        self.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, v)| (Pos::new(i, j), v))
        })
    }
}

#[snippet = "grid"]
impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index]
    }
}

#[snippet = "grid"]
impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index]
    }
}

#[snippet = "grid"]
impl<T> std::ops::Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.x][index.y]
    }
}

#[snippet = "grid"]
impl<T> std::ops::IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}

#[snippet = "vec_util"]
trait VecUtil<T> {
    fn get_or_default(&self, i: usize) -> T;
}

#[snippet = "vec_util"]
impl<T: Default + Copy> VecUtil<T> for Vec<T> {
    fn get_or_default(self: &Vec<T>, i: usize) -> T {
        self.get(i).copied().unwrap_or_default()
    }
}

#[snippet = "direction"]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
enum Direction {
    #[default]
    U,
    D,
    L,
    R,
    S,
}

#[snippet = "direction"]
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = match self {
            Self::U => "U",
            Self::D => "D",
            Self::L => "L",
            Self::R => "R",
            Self::S => "S",
        };

        write!(f, "{}", v)?;
        Ok(())
    }
}

#[snippet = "direction"]
impl Direction {
    fn to_unsafe_pos(&self) -> UnsafePos {
        match self {
            Self::U => UnsafePos::new(-1, 0),
            Self::D => UnsafePos::new(1, 0),
            Self::L => UnsafePos::new(0, -1),
            Self::R => UnsafePos::new(0, 1),
            Self::S => UnsafePos::new(0, 0),
        }
    }
}
