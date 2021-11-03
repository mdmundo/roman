use std::collections::HashMap;

const THOUSANDS: [&str; 3] = ["MMM", "MM", "M"];

const HUNDREDS: [&str; 9] = ["CM", "DCCC", "DCC", "DC", "D", "CD", "CCC", "CC", "C"];

const TENS: [&str; 9] = ["XC", "LXXX", "LXX", "LX", "L", "XL", "XXX", "XX", "X"];

const UNITS: [&str; 9] = ["IX", "VIII", "VII", "VI", "V", "IV", "III", "II", "I"];

enum Types {
    Thousands,
    Hundreds,
    Tens,
    Units,
}

fn is_it(kind: Types, roman: &str) -> bool {
    let result = match kind {
        Types::Thousands => roman.find('M'),
        Types::Hundreds => roman.find(|c: char| c == 'C' || c == 'D'),
        Types::Tens => roman.find(|c: char| c == 'X' || c == 'L'),
        Types::Units => roman.find(|c: char| c == 'I' || c == 'V'),
    };
    result.is_some()
}

fn thousands<'a>(roman: &'a str, table: &HashMap<String, u16>) -> (u16, &'a str) {
    let mut num: u16 = 0;
    let mut piece: &str = roman;
    match is_it(Types::Thousands, roman) {
        true => {
            for place in THOUSANDS {
                match roman.strip_prefix(place) {
                    Some(roman) => {
                        num = *table.get(place).unwrap();
                        piece = roman;
                        break;
                    }
                    _ => continue,
                }
            }
        }
        false => (),
    }
    (num, piece)
}

fn hundreds<'a>(roman: &'a str, table: &HashMap<String, u16>) -> (u16, &'a str) {
    let mut num: u16 = 0;
    let mut piece: &str = roman;
    match is_it(Types::Hundreds, roman) {
        true => {
            for place in HUNDREDS {
                match roman.strip_prefix(place) {
                    Some(roman) => {
                        num = *table.get(place).unwrap();
                        piece = roman;
                        break;
                    }
                    _ => continue,
                }
            }
        }
        false => (),
    }
    (num, piece)
}

fn tens<'a>(roman: &'a str, table: &HashMap<String, u16>) -> (u16, &'a str) {
    let mut num: u16 = 0;
    let mut piece: &str = roman;
    match is_it(Types::Tens, roman) {
        true => {
            for place in TENS {
                match roman.strip_prefix(place) {
                    Some(roman) => {
                        num = *table.get(place).unwrap();
                        piece = roman;
                        break;
                    }
                    _ => continue,
                }
            }
        }
        false => (),
    }
    (num, piece)
}

fn units<'a>(roman: &'a str, table: &HashMap<String, u16>) -> (u16, &'a str) {
    let mut num: u16 = 0;
    let mut piece: &str = roman;
    match is_it(Types::Units, roman) {
        true => {
            for place in UNITS {
                match roman.strip_prefix(place) {
                    Some(roman) => {
                        num = *table.get(place).unwrap();
                        piece = roman;
                        break;
                    }
                    _ => continue,
                }
            }
        }
        false => (),
    }
    (num, piece)
}

pub struct Numeral {
    pub roman: String,
    pub table: HashMap<String, u16>,
}

impl Numeral {
    pub fn new(roman: String) -> Result<Numeral, String> {
        if roman.is_empty() {
            return Err("Roman numeral required".to_owned());
        }

        let table: HashMap<String, u16> = HashMap::from([
            ("MMM".to_owned(), 3000),
            ("MM".to_owned(), 2000),
            ("M".to_owned(), 1000),
            ("CM".to_owned(), 900),
            ("DCCC".to_owned(), 800),
            ("DCC".to_owned(), 700),
            ("DC".to_owned(), 600),
            ("D".to_owned(), 500),
            ("CD".to_owned(), 400),
            ("CCC".to_owned(), 300),
            ("CC".to_owned(), 200),
            ("C".to_owned(), 100),
            ("XC".to_owned(), 90),
            ("LXXX".to_owned(), 80),
            ("LXX".to_owned(), 70),
            ("LX".to_owned(), 60),
            ("L".to_owned(), 50),
            ("XL".to_owned(), 40),
            ("XXX".to_owned(), 30),
            ("XX".to_owned(), 20),
            ("X".to_owned(), 10),
            ("IX".to_owned(), 9),
            ("VIII".to_owned(), 8),
            ("VII".to_owned(), 7),
            ("VI".to_owned(), 6),
            ("V".to_owned(), 5),
            ("IV".to_owned(), 4),
            ("III".to_owned(), 3),
            ("II".to_owned(), 2),
            ("I".to_owned(), 1),
        ]);

        Ok(Numeral { roman, table })
    }
}

pub fn run(numeral: &Numeral) -> Result<u16, String> {
    let (result_thousands, roman) = thousands(&numeral.roman, &numeral.table);
    let (result_hundreds, roman) = hundreds(&roman, &numeral.table);
    let (result_tens, roman) = tens(&roman, &numeral.table);
    let (result_units, _) = units(&roman, &numeral.table);
    let result = result_thousands + result_hundreds + result_tens + result_units;
    Ok(result)
}
