use std::collections::HashMap;

pub struct Numeral {
    pub roman: String,
    pub table: HashMap<&'static str, u16>,
    pub thousands: Vec<&'static str>,
    pub hundreds: Vec<&'static str>,
    pub tens: Vec<&'static str>,
    pub units: Vec<&'static str>,
}

impl Numeral {
    pub fn new(roman: String) -> Result<Numeral, &'static str> {
        if roman.is_empty() {
            return Err("Roman numeral required");
        }

        let roman = roman.to_uppercase();

        let mmm = "MMM";
        let mm = "MM";
        let m = "M";
        let cm = "CM";
        let dccc = "DCCC";
        let dcc = "DCC";
        let dc = "DC";
        let d = "D";
        let cd = "CD";
        let ccc = "CCC";
        let cc = "CC";
        let c = "C";
        let xc = "XC";
        let lxxx = "LXXX";
        let lxx = "LXX";
        let lx = "LX";
        let l = "L";
        let xl = "XL";
        let xxx = "XXX";
        let xx = "XX";
        let x = "X";
        let ix = "IX";
        let viii = "VIII";
        let vii = "VII";
        let vi = "VI";
        let v = "V";
        let iv = "IV";
        let iii = "III";
        let ii = "II";
        let i = "I";

        let table: HashMap<&str, u16> = HashMap::from([
            (mmm, 3000),
            (mm, 2000),
            (m, 1000),
            (cm, 900),
            (dccc, 800),
            (dcc, 700),
            (dc, 600),
            (d, 500),
            (cd, 400),
            (ccc, 300),
            (cc, 200),
            (c, 100),
            (xc, 90),
            (lxxx, 80),
            (lxx, 70),
            (lx, 60),
            (l, 50),
            (xl, 40),
            (xxx, 30),
            (xx, 20),
            (x, 10),
            (ix, 9),
            (viii, 8),
            (vii, 7),
            (vi, 6),
            (v, 5),
            (iv, 4),
            (iii, 3),
            (ii, 2),
            (i, 1),
        ]);

        let thousands = vec![mmm, mm, m];
        let hundreds = vec![cm, dccc, dcc, dc, d, cd, ccc, cc, c];
        let tens = vec![xc, lxxx, lxx, lx, l, xl, xxx, xx, x];
        let units = vec![ix, viii, vii, vi, v, iv, iii, ii, i];

        Ok(Numeral {
            roman,
            table,
            thousands,
            hundreds,
            tens,
            units,
        })
    }
}

enum Types {
    Thousands,
    Hundreds,
    Tens,
    Units,
}

fn is_it(kind: &Types, roman: &str) -> bool {
    let result = match kind {
        Types::Thousands => roman.find('M'),
        Types::Hundreds => roman.find(|c: char| c == 'C' || c == 'D'),
        Types::Tens => roman.find(|c: char| c == 'X' || c == 'L'),
        Types::Units => roman.find(|c: char| c == 'I' || c == 'V'),
    };
    result.is_some()
}

fn get<'a>(
    roman: &'a str,
    table: &HashMap<&str, u16>,
    kind: &Types,
    array: &Vec<&str>,
) -> (u16, &'a str) {
    let mut num: u16 = 0;
    let mut piece: &str = roman;
    match is_it(kind, roman) {
        true => {
            for place in array {
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

pub fn run(numeral: &Numeral) -> Result<u16, &str> {
    let (result_thousands, roman) = get(
        &numeral.roman,
        &numeral.table,
        &Types::Thousands,
        &numeral.thousands,
    );
    let (result_hundreds, roman) = get(&roman, &numeral.table, &Types::Hundreds, &numeral.hundreds);
    let (result_tens, roman) = get(&roman, &numeral.table, &Types::Tens, &numeral.tens);
    let (result_units, roman) = get(&roman, &numeral.table, &Types::Units, &numeral.units);
    let result = result_thousands + result_hundreds + result_tens + result_units;
    if !roman.is_empty() {
        Err("Invalid input")
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_thousands() {
        let roman = "MMMCMXCIX".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = get(
            &numeral.roman,
            &numeral.table,
            &Types::Thousands,
            &numeral.thousands,
        );

        assert_eq!((3_000, "CMXCIX"), result);
    }

    #[test]
    fn one() {
        let roman = "I".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = get(
            &numeral.roman,
            &numeral.table,
            &Types::Units,
            &numeral.units,
        );

        assert_eq!((1, ""), result);
    }

    #[test]
    fn not_units() {
        let roman = "IIII".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = get(
            &numeral.roman,
            &numeral.table,
            &Types::Units,
            &numeral.units,
        );

        assert_eq!((3, "I"), result);
    }

    #[test]
    fn not_hundreds() {
        let roman = "MMMCMXCIX".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = get(
            &numeral.roman,
            &numeral.table,
            &Types::Hundreds,
            &numeral.hundreds,
        );

        assert_eq!((0, "MMMCMXCIX"), result);
    }

    #[test]
    fn correct_run() {
        let roman = "MMMCMXCIX".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = run(&numeral);

        assert_eq!(3_999, result.unwrap());
    }

    #[test]
    fn incorrect_run() {
        let roman = "MMMCMXCX".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = run(&numeral);

        assert_eq!(Err("Invalid input"), result);
    }

    #[test]
    fn greatest_roman_str_run() {
        let roman = "MMMDCCCLXXXVIII".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = run(&numeral);

        assert_eq!(Ok(3_888), result);
    }

    #[test]
    fn lowercase_run() {
        let roman = "mmmdccclxxxviii".to_owned();
        let numeral = Numeral::new(roman).unwrap();
        let result = run(&numeral);

        assert_eq!(Ok(3_888), result);
    }
}
