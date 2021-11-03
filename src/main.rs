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

    let roman = "MMMCMXCIX";
    let mut decimal: u16 = 0;
    for place in THOUSANDS {
        println!("---");
        println!("place: {}", place);
        println!(
            "roman.strip_prefix(place).unwrap(): {}",
            roman.strip_prefix(place).unwrap()
        );
        match roman.strip_prefix(place) {
            Some(roman) => {
                println!("roman: {}", roman);
                decimal += table.get(place).unwrap();
                assert_eq!(decimal, 3_000);
                // call next for loop

                for place in HUNDREDS {
                    println!("---");
                    println!("place: {}", place);
                    println!(
                        "roman.strip_prefix(place).unwrap(): {}",
                        roman.strip_prefix(place).unwrap()
                    );
                    match roman.strip_prefix(place) {
                        Some(roman) => {
                            println!("roman: {}", roman);
                            decimal += table.get(place).unwrap();
                            assert_eq!(decimal, 3_900);
                            // call next for loop

                            for place in TENS {
                                println!("---");
                                println!("place: {}", place);
                                println!(
                                    "roman.strip_prefix(place).unwrap(): {}",
                                    roman.strip_prefix(place).unwrap()
                                );
                                match roman.strip_prefix(place) {
                                    Some(roman) => {
                                        println!("roman: {}", roman);
                                        decimal += table.get(place).unwrap();
                                        assert_eq!(decimal, 3_990);
                                        // call next for loop

                                        for place in UNITS {
                                            println!("---");
                                            println!("place: {}", place);
                                            println!(
                                                "roman.strip_prefix(place).unwrap(): {}",
                                                roman.strip_prefix(place).unwrap()
                                            );
                                            match roman.strip_prefix(place) {
                                                Some(roman) => {
                                                    println!("roman: {}", roman);
                                                    decimal += table.get(place).unwrap();
                                                    assert_eq!(decimal, 3_999);
                                                    // call next for loop
                                                    break;
                                                }
                                                _ => break,
                                            }
                                        }

                                        break;
                                    }
                                    _ => break,
                                }
                            }

                            break;
                        }
                        _ => break,
                    }
                }

                break;
            }
            _ => break,
        }
    }
}
