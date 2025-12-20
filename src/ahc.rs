#[derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Pos {
    x: usize,
    y: usize
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnsafePos {
    x: isize,
    y: isize
}

impl UnsafePos {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    
    fn is_valid(&self, n: isize) -> bool {
        0 <= self.x && self.x < n && 0 <= self.y && self.y < n
    }
}

struct Grid<T> {
    g: Vec<Vec<T>>
}

impl<T: Clone> Grid<T> {
    fn new(n: usize) -> Self {
        Self { g: vec![vec![];n] }
    }

    fn new_with_default(n: usize, d: T) -> Self {
        Self { g: vec![vec![d;n];n] }
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.g.iter()
    }

    fn pos_iter(&self) -> impl Iterator<Item = (Pos, &T)> {
        self.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, v)| (Pos::new(i, j), v))
        })
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index]
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.x][index.y]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}
trait VecUtil<T> {
    fn get_or_default(&self, i: usize) -> T;
}

impl<T: Default + Copy> VecUtil<T> for Vec<T> {
    fn get_or_default(self: &Vec<T>, i: usize) -> T {
        self.get(i).copied().unwrap_or_default()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
enum Direction {
    #[default]
    U,
    D,
    L,
    R,
    S,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let v = match self {
            Self::U => "U",
            Self::D => "D",
            Self::L => "L",
            Self::R => "R",
            Self::S => "S"
        };

        write!(f, "{}", v)?;
        Ok(())
    }
}

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