use roman::{run, Numeral};

fn main() {
    let numeral = Numeral::new("MVIII".to_owned()).unwrap();
    let result = run(numeral).unwrap();
    // thousands(&mut decimal, roman, &table);
    assert_eq!(result, 1_008);
}
