use cargo_snippet_more::{snippet, snippet_end, snippet_start};

snippet_start!("pos");
#[proconio::derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn new_if_is_valid(&self, d: UnsafePos, n: usize) -> Option<Self> {
        let next = UnsafePos::new(self.x as isize + d.x, self.y as isize + d.y);
        if next.is_valid(n as isize) {
            Some(Self::new(next.x as usize, next.y as usize))
        } else {
            None
        }
    }
}

snippet_start!("unsafe_pos");
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnsafePos {
    x: isize,
    y: isize,
}

impl UnsafePos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn is_valid(&self, n: isize) -> bool {
        0 <= self.x && self.x < n && 0 <= self.y && self.y < n
    }
}
snippet_end!("unsafe_pos");
snippet_end!("pos");

snippet_start!(name = "grid", include = "pos");
pub struct Grid<T> {
    g: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(n: usize) -> Self {
        Self { g: vec![vec![]; n] }
    }

    pub fn new_with_default(n: usize, d: T) -> Self {
        Self {
            g: vec![vec![d; n]; n],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.g.iter()
    }

    pub fn pos_iter(&self) -> impl Iterator<Item = (Pos, &T)> {
        self.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, v)| (Pos::new(i, j), v))
        })
    }
}

#[snippet("grid")]
impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index]
    }
}

#[snippet("grid")]
impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index]
    }
}

#[snippet("grid")]
impl<T> std::ops::Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.x][index.y]
    }
}

#[snippet("grid")]
impl<T> std::ops::IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}
snippet_end!("grid");

snippet_start!("vec_util");
pub trait VecUtil<T> {
    fn get_or_default(&self, i: usize) -> T;
}

impl<T: Default + Copy> VecUtil<T> for Vec<T> {
    fn get_or_default(self: &Vec<T>, i: usize) -> T {
        self.get(i).copied().unwrap_or_default()
    }
}
snippet_end!("vec_util");

snippet_start!("direction");
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
pub enum Direction {
    #[default]
    U,
    D,
    L,
    R,
    S,
}

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

impl Direction {
    pub fn to_unsafe_pos(&self) -> UnsafePos {
        match self {
            Self::U => UnsafePos::new(-1, 0),
            Self::D => UnsafePos::new(1, 0),
            Self::L => UnsafePos::new(0, -1),
            Self::R => UnsafePos::new(0, 1),
            Self::S => UnsafePos::new(0, 0),
        }
    }
}
snippet_end!("direction");
