use std::collections::HashMap;

const THOUSANDS: [&str; 3] = ["MMM", "MM", "M"];

const HUNDREDS: [&str; 9] = ["CM", "DCCC", "DCC", "DC", "D", "CD", "CCC", "CC", "C"];

const TENS: [&str; 9] = ["XC", "LXXX", "LXX", "LX", "L", "XL", "XXX", "XX", "X"];

const UNITS: [&str; 9] = ["IX", "VIII", "VII", "VI", "V", "IV", "III", "II", "I"];

fn main() {
    let table: HashMap<&str, u16> = HashMap::from([
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

    let roman = "MC";
    let mut decimal: u16 = 0;
    thousands(&mut decimal, roman, &table);
    assert_eq!(decimal, 1_100);
}

fn thousands(decimal: &mut u16, roman: &str, table: &HashMap<&str, u16>) {
    match is_thousands(roman) {
        Some(_) => {
            for place in THOUSANDS {
                match roman.strip_prefix(place) {
                    Some(roman) => {
                        *decimal += table.get(place).unwrap();
                        hundreds(decimal, roman, table);
                        break;
                    }
                    _ => continue,
                }
            }
        }
        None => hundreds(decimal, roman, table),
    }
}

fn is_thousands(roman: &str) -> Option<usize> {
    roman.find('M')
}

fn hundreds(decimal: &mut u16, roman: &str, table: &HashMap<&str, u16>) {
    match is_hundreds(roman) {
        Some(_) => {
            for place in HUNDREDS {
                match roman.strip_prefix(place) {
                    Some(roman) => {
                        *decimal += table.get(place).unwrap();
                        break;
                    }
                    _ => continue,
                }
            }
        }
        None => (),
    }
}

fn is_hundreds(roman: &str) -> Option<usize> {
    roman.find(|c: char| c == 'C' || c == 'D')
}
