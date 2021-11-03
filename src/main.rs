use roman::{run, Numeral};

fn main() {
    let numeral = Numeral::new("MVIII".to_owned()).unwrap();
    let result = run(numeral).unwrap();
    assert_eq!(result, 1_008);
}
