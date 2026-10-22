use cargo_snippet_more::{p, snippet, snippet_end, snippet_start};

#[snippet(name = "math", not_library)]
#[snippet]
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[snippet(include = "gcd")]
pub fn gcd_list(list: &[usize]) -> usize {
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}

#[snippet(name = "math", not_library)]
#[snippet]
pub fn lcm(a: usize, b: usize) -> usize {
    p!(1, a) / gcd(a, b) * b
}

#[snippet]
pub fn to_base(mut n: usize, b: usize) -> usize {
    let mut v = String::new();
    while n != 0 {
        v.push_str(&(n % b).to_string());
        n /= b;
    }

    v.chars().rev().collect::<String>().parse().unwrap()
}

#[snippet]
pub fn count<T: Ord + Copy>(v: &[T]) -> std::collections::BTreeMap<T, usize> {
    let mut map = std::collections::BTreeMap::new();
    for i in v.iter().copied() {
        *map.entry(i).or_insert(0) += 1;
    }

    map
}

#[snippet]
pub fn sum_vec<T: std::ops::Add<Output = T> + Copy>(v: &[T]) -> Vec<T> {
    let mut n = vec![v[0]];
    for (i, e) in v.iter().skip(1).copied().enumerate() {
        n.push(n[i] + e);
    }

    n
}

snippet_start!(name = "cast_ext", library = "CastExt");
pub trait CastExt {
    fn i(self) -> isize;
    fn u(self) -> usize;
}

macro_rules! impl_numeric_cast {
    ($($t:ty),*) => {
        $(
            impl CastExt for $t {
                #[inline]
                fn i(self) -> isize { self as isize }
                #[inline]
                fn u(self) -> usize { self as usize }
            }
        )*
    };
}

impl_numeric_cast!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl CastExt for char {
    #[inline]
    fn i(self) -> isize {
        self.to_digit(10).unwrap().i()
    }

    #[inline]
    fn u(self) -> usize {
        self.to_digit(10).unwrap().u()
    }
}
snippet_end!("cast_ext");

#[snippet]
#[macro_export]
macro_rules! update_max {
    ($a:expr) => {
    };

    ($a:expr, $b: expr $(, $rest:expr)*) => {
        {
            $a = $a.max($b);
            update_max!($a $(, $rest)*);
        }
    }
}

#[snippet]
#[macro_export]
macro_rules! update_min {
    ($a:expr) => {
    };

    ($a:expr, $b: expr $(, $rest:expr)*) => {
        {
            $a = $a.min($b);
            update_min!($a $(, $rest)*);
        }
    }
}

#[test]
fn to_base_test() {
    let two = to_base(10, 2);
    assert_eq!(two, 1010);

    let three = to_base(10, 3);
    assert_eq!(three, 101);
}
