use cargo_snippet_more::{p, snippet};

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

#[test]
fn to_base_test() {
    let two = to_base(10, 2);
    assert_eq!(two, 1010);

    let three = to_base(10, 3);
    assert_eq!(three, 101);
}
