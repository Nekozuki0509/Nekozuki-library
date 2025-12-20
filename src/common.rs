use cargo_snippet::snippet;

# [snippet("math")]
# [snippet]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

# [snippet("math")]
# [snippet]
# [snippet(include = "gcd")]
pub fn gcd_list(list: &[u64]) -> u64 {
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}

# [snippet("math")]
# [snippet]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}