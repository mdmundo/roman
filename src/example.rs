fn main() {
    let mut s = String::from("MMCDXXI");
    let disjoint_match = s.match_indices("MM").next().unwrap_or((0, ""));
    let offset = disjoint_match.0 + disjoint_match.1.len();

    s.replace_range(..offset, "");
    assert_eq!(s, "CDXXI");
}
