use cargo_snippet_more::snippet;

#[snippet(name = "math", not_library)]
#[snippet]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[snippet]
#[snippet(include = "gcd")]
pub fn gcd_list(list: &[u64]) -> u64 {
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}

#[snippet(name = "math", not_library)]
#[snippet]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
