use roman::{run, Numeral};

fn main() {
    let mut args = std::env::args().skip(1);
    let roman = args.next().expect("Numeral required");
    let numeral = Numeral::new(&roman).unwrap();
    let result = run(&numeral).unwrap();
    println!("Roman {} is decimal {}.", numeral.roman, result);
}
