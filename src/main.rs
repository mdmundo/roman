use std::collections::HashMap;

const ROMAN: [&str; 30] = [
    "MMM", "MM", "M", "CM", "DCCC", "DCC", "DC", "D", "CD", "CCC", "CC", "C", "XC", "LXXX", "LXX",
    "LX", "L", "XL", "XXX", "XX", "X", "IX", "VIII", "VII", "VI", "V", "IV", "III", "II", "I",
];

//     Thousands   Hundreds    Tens    Units
// 1   M           C           X       I
// 2   MM          CC          XX      II
// 3   MMM         CCC         XXX     III
// 4               CD          XL      IV
// 5               D           L       V
// 6               DC          LX      VI
// 7               DCC         LXX     VII
// 8               DCCC        LXXX    VIII
// 9               CM          XC      IX

fn main() {
    let table: HashMap<&str, u32> = HashMap::from([
        ("MMM", 3000),
        ("MM", 2000),
        ("M", 1000),
        ("CM", 900),
        ("DCCC", 800),
        ("DCC", 700),
        ("DC", 600),
        ("D", 500),
        ("CD", 400),
        ("CCC", 300),
        ("CC", 200),
        ("C", 100),
        ("XC", 90),
        ("LXXX", 80),
        ("LXX", 70),
        ("LX", 60),
        ("L", 50),
        ("XL", 40),
        ("XXX", 30),
        ("XX", 20),
        ("X", 10),
        ("IX", 9),
        ("VIII", 8),
        ("VII", 7),
        ("VI", 6),
        ("V", 5),
        ("IV", 4),
        ("III", 3),
        ("II", 2),
        ("I", 1),
    ]);
    let mut roman = String::from("MMCDXXI");
    let mut decimal: u32 = 0;
    // let decimal = match roman {
    //     "MMM" => 3000,
    //     _ => 0,
    // };
    for place in ROMAN {
        match roman.matches(place).next() {
            // Add actual value later
            // Some(val) => decimal += 1,
            Some(val) => {
                let disjoint_match = roman.match_indices(val).next().unwrap();
                let offset = disjoint_match.0 + disjoint_match.1.len();
                roman.replace_range(..offset, "");
                decimal += table.get(val).unwrap();
            }
            None => continue,
        }
    }
    assert_eq!(roman, "");
    assert_eq!(decimal, 2421);
}
