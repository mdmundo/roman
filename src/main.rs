use roman::{run, Numeral};

fn main() {
    let numeral = Numeral::new("MM".to_owned()).unwrap();
    let result = run(numeral).unwrap();
    // thousands(&mut decimal, roman, &table);
    assert_eq!(result, 2_000);
}
