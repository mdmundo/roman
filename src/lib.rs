use std::collections::HashMap;

const THOUSANDS: [&str; 3] = ["MMM", "MM", "M"];

const HUNDREDS: [&str; 9] = ["CM", "DCCC", "DCC", "DC", "D", "CD", "CCC", "CC", "C"];

const TENS: [&str; 9] = ["XC", "LXXX", "LXX", "LX", "L", "XL", "XXX", "XX", "X"];

const UNITS: [&str; 9] = ["IX", "VIII", "VII", "VI", "V", "IV", "III", "II", "I"];

fn thousands(decimal: &mut u16, roman: &str, table: &HashMap<&str, u16>) {
    match is_thousands(roman) {
        Some(_) => {
            for place in THOUSANDS {
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
