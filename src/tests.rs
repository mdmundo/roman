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
fn another_incorrect_run() {
    let roman = "qwerty".to_owned();
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
