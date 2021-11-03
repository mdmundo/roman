use std::collections::HashMap;

const THOUSANDS: [&str; 3] = ["MMM", "MM", "M"];

const HUNDREDS: [&str; 9] = ["CM", "DCCC", "DCC", "DC", "D", "CD", "CCC", "CC", "C"];

const TENS: [&str; 9] = ["XC", "LXXX", "LXX", "LX", "L", "XL", "XXX", "XX", "X"];

const UNITS: [&str; 9] = ["IX", "VIII", "VII", "VI", "V", "IV", "III", "II", "I"];

fn thousands<'a>(roman: &'a str, table: &HashMap<String, u16>) -> (u16, &'a str) {
    let mut num: u16 = 0;
    let mut piece: &str = roman;
    match is_thousands(roman) {
        Some(_) => {
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
        None => (),
    }
    (num, piece)
}

fn is_thousands(roman: &str) -> Option<usize> {
    roman.find('M')
}

// fn hundreds(decimal: &mut u16, roman: &str) {
//     match is_hundreds(roman) {
//         Some(_) => {
//             for place in HUNDREDS {
//                 match roman.strip_prefix(place) {
//                     Some(roman) => {
//                         *decimal += table.get(place).unwrap();
//                         break;
//                     }
//                     _ => continue,
//                 }
//             }
//         }
//         None => (),
//     }
// }

// fn is_hundreds(roman: &str) -> Option<usize> {
//     roman.find(|c: char| c == 'C' || c == 'D')
// }

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

pub fn run(numeral: Numeral) -> Result<u16, String> {
    Ok(thousands(&numeral.roman, &numeral.table).0)
}

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value.
//     // Expressions implicitly return the unit value if they don’t return any other value.

//     let contents = fs::read_to_string(config.filename)?;
//     // A Shortcut for Propagating Errors: the ? Operator
//     // Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.

//     let results = if config.case_sensitive {
//         search(&config.query, &contents)
//     } else {
//         search_case_insensitive(&config.query, &contents)
//     };

//     for line in results {
//         println!("{}", line);
//     }

//     Ok(())
// }

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     // Lifetime Annotation Syntax
//     // &i32        // a reference
//     // &'a i32     // a reference with an explicit lifetime
//     // &'a mut i32 // a mutable reference with an explicit lifetime
//     // The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.
//     // The first rule is that each parameter that is a reference gets its own lifetime parameter.
//     // The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
//     // The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }

// pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }

//     results
// }
