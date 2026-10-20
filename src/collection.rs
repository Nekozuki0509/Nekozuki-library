use cargo_snippet_more::{snippet, snippet_end, snippet_start};

snippet_start!(name = "pos", include = "unsafe_pos", library = "Pos");
#[proconio::derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to_unsafe(&self) -> UnsafePos {
        UnsafePos::new(self.x as isize, self.y as isize)
    }

    pub fn new_if_is_valid(&self, d: UnsafePos, n: usize, m: usize) -> Option<Self> {
        let next = self.addu(d);
        if next.is_valid(n as isize, m as isize) {
            Some(next.as_pos())
        } else {
            None
        }
    }

    pub fn addu(&self, rhs: UnsafePos) -> UnsafePos {
        self.to_unsafe() + rhs
    }

    pub fn subu(&self, rhs: UnsafePos) -> UnsafePos {
        self.to_unsafe() - rhs
    }

    pub fn add(&self, rhs: (usize, usize)) -> Pos {
        *self + Pos::from(rhs)
    }

    pub fn sub(&self, rhs: (usize, usize)) -> UnsafePos {
        self.to_unsafe().sub(rhs)
    }

    pub fn addi(&self, rhs: (isize, isize)) -> UnsafePos {
        self.to_unsafe().addi(rhs)
    }

    pub fn subi(&self, rhs: (isize, isize)) -> UnsafePos {
        self.to_unsafe().subi(rhs)
    }

    pub fn incx(&self) -> Pos {
        self.to_unsafe().incx().as_pos()
    }

    pub fn decx(&self) -> UnsafePos {
        self.to_unsafe().decx()
    }

    pub fn incy(&self) -> Pos {
        self.to_unsafe().incy().as_pos()
    }

    pub fn decy(&self) -> UnsafePos {
        self.to_unsafe().decy()
    }

    pub fn squr_dist(&self, other: &Pos) -> usize {
        self.to_unsafe().squr_dist(&other.to_unsafe())
    }

    pub fn dist(&self, other: &Pos) -> usize {
        num_integer::sqrt(self.squr_dist(other))
    }

    pub fn fdist(&self, other: &Pos) -> f64 {
        self.to_unsafe().fdist(&other.to_unsafe())
    }

    pub fn iter(&self) -> IPos {
        IPos::new(Pos::default(), *self)
    }

    pub fn range(&self, end: Pos) -> IPos {
        if let Some(end) = end.new_if_is_valid(UnsafePos::new(-1, -1), end.x, end.y) {
            IPos::new(*self, end)
        } else {
            IPos::default()
        }
    }

    pub fn range_eq(&self, end: Pos) -> IPos {
        IPos::new(*self, end)
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        UnsafePos::from(value).as_pos()
    }
}

impl From<(isize, isize)> for Pos {
    fn from(value: (isize, isize)) -> Self {
        UnsafePos::from(value).as_pos()
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Pos {
    type Output = UnsafePos;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subu(rhs.to_unsafe())
    }
}

impl IntoIterator for Pos {
    type Item = Pos;
    type IntoIter = IPos;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Default)]
pub struct IPos {
    iter: IUnsafePos,
}

impl IPos {
    fn new(start: Pos, end: Pos) -> IPos {
        IPos {
            iter: IUnsafePos::new(start.to_unsafe(), end.to_unsafe()),
        }
    }
}

impl Iterator for IPos {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| p.as_pos())
    }
}
snippet_end!("pos");

snippet_start!(name = "unsafe_pos", include = "pos", library = "UnsafePos");
#[proconio::derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UnsafePos {
    x: isize,
    y: isize,
}

impl UnsafePos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn as_pos(&self) -> Pos {
        Pos::new(self.x as usize, self.y as usize)
    }

    pub fn is_valid(&self, n: isize, m: isize) -> bool {
        0 <= self.x && self.x < n && 0 <= self.y && self.y < m
    }

    pub fn add(&self, rhs: (usize, usize)) -> Self {
        *self + Self::from(rhs)
    }

    pub fn sub(&self, rhs: (usize, usize)) -> Self {
        *self - Self::from(rhs)
    }

    pub fn addi(&self, rhs: (isize, isize)) -> Self {
        *self + Self::from(rhs)
    }

    pub fn subi(&self, rhs: (isize, isize)) -> Self {
        *self - Self::from(rhs)
    }

    pub fn incx(&self) -> Self {
        self.add((1, 0))
    }

    pub fn decx(&self) -> Self {
        self.sub((1, 0))
    }

    pub fn incy(&self) -> Self {
        self.add((0, 1))
    }

    pub fn decy(&self) -> Self {
        self.sub((0, 1))
    }

    pub fn squr_dist(&self, other: &UnsafePos) -> usize {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as usize
    }

    pub fn dist(&self, other: &UnsafePos) -> usize {
        num_integer::sqrt(self.squr_dist(other))
    }

    pub fn fdist(&self, other: &UnsafePos) -> f64 {
        num::Float::sqrt(
            (self.x as f64 - other.x as f64).powf(2.) + (self.y as f64 - other.y as f64).powf(2.),
        )
    }

    pub fn iter(&self) -> IUnsafePos {
        IUnsafePos::new(UnsafePos::default(), *self)
    }

    pub fn range(&self, end: Self) -> IUnsafePos {
        IUnsafePos::new(*self, end.decx().decy())
    }

    pub fn range_eq(&self, end: Self) -> IUnsafePos {
        IUnsafePos::new(*self, end)
    }
}

impl From<(usize, usize)> for UnsafePos {
    fn from(value: (usize, usize)) -> Self {
        UnsafePos::new(value.0 as isize, value.1 as isize)
    }
}

impl From<(isize, isize)> for UnsafePos {
    fn from(value: (isize, isize)) -> Self {
        UnsafePos::new(value.0, value.1)
    }
}

impl std::ops::Add for UnsafePos {
    type Output = UnsafePos;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for UnsafePos {
    type Output = UnsafePos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.y, self.x - rhs.y)
    }
}

#[derive(Default)]
pub struct IUnsafePos {
    start: UnsafePos,
    end: UnsafePos,
    cur: UnsafePos,
}

impl IUnsafePos {
    fn new(start: UnsafePos, end: UnsafePos) -> Self {
        Self {
            start,
            end,
            cur: start,
        }
    }
}

impl Iterator for IUnsafePos {
    type Item = UnsafePos;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.cur.incy();
        if t.y <= self.end.y {
            self.cur = t;
            return Some(self.cur);
        }

        let t = UnsafePos::new(self.cur.x + 1, self.start.y);
        if t.x <= self.end.x {
            self.cur = t;
            return Some(self.cur);
        }

        None
    }
}
snippet_end!("unsafe_pos");

snippet_start!(name = "grid", include = "pos", library = "Grid");
pub struct Grid<T> {
    g: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn pos_iter(&self) -> impl Iterator<Item = (Pos, &T)> {
        self.g.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, v)| (Pos::new(i, j), v))
        })
    }

    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, Vec<T>>> {
        self.g.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.g.iter_mut().flat_map(|r| r.iter_mut())
    }
}
impl<T: Clone> Grid<T> {
    pub fn new(n: usize, m: usize, d: T) -> Self {
        Self {
            g: vec![vec![d; m]; n],
        }
    }

    pub fn rotate_right(&self) -> Self {
        let mut g = Grid::new(self[0].len(), self.g.len(), self[0][0].clone());

        for (p, v) in self.pos_iter() {
            let y = g[0].len() - p.x - 1;
            g[p.y][y] = v.clone();
        }

        g
    }

    pub fn rotate_left(&self) -> Self {
        let mut g = Grid::new(self.g[0].len(), self.g.len(), self[0][0].clone());

        for (p, v) in self.pos_iter() {
            let x = g.g.len() - p.y - 1;
            g[x][p.x] = v.clone();
        }

        g
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(g: Vec<Vec<T>>) -> Self {
        Self { g }
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index]
    }
}

impl<T> std::ops::Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.x][index.y]
    }
}

impl<T> std::ops::IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::iter::Flatten<std::vec::IntoIter<Vec<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.g.into_iter().flatten()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, Vec<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
snippet_end!("grid");

snippet_start!(name = "vec_util", library = "VecUtil");
pub trait VecUtil<T> {
    fn get_or_default(&self, i: usize) -> T;
}

impl<T: Default + Copy> VecUtil<T> for Vec<T> {
    fn get_or_default(self: &Vec<T>, i: usize) -> T {
        self.get(i).copied().unwrap_or_default()
    }
}
snippet_end!("vec_util");

snippet_start!(
    name = "direction",
    library = "Direction",
    include = "unsafe_pos"
);
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

#[snippet]
#[macro_export]
macro_rules! set {
    ($($x:expr),* $(,)?) => {
        {
            let mut set = ::std::collections::BTreeSet::new();
            $(
                set.insert($x);
            )*

            set
        }
    };
}

#[snippet]
#[macro_export]
macro_rules! map {
    ($($key:expr=>$val:expr),* $(,)?) => {
        {
            let mut map = ::std::collections::BTreeMap::new();
            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

#[snippet]
#[macro_export]
macro_rules! que {
    ($($x:expr),* $(,)?) => {
        {
            let mut que = ::std::collections::VecDeque::new();
            $(
                que.push_back($x);
            )*

            que
        }
    };
}

#[snippet]
#[macro_export]
macro_rules! heap {
    ($($x:expr),* $(,)?) => {
        {
            let mut heap = ::std::collections::BinaryHeap::new();
            $(
                heap.push($x);
            )*

            heap
        }
    };
}
