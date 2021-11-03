// const ROMAN: [&str; 30] = [
//     "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XX", "XXX", "XL", "L", "LX",
//     "LXX", "LXXX", "XC", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM", "M", "MM", "MMM",
// ];
const ROMAN: [&str; 30] = [
    "MMM", "MM", "M", "CM", "DCCC", "DCC", "DC", "D", "CD", "CCC", "CC", "C", "XC", "LXXX", "LXX",
    "LX", "L", "XL", "XXX", "XX", "X", "IX", "VIII", "VII", "VI", "V", "IV", "III", "II", "I",
];

//     Thousands   Hundreds    Tens    Units
// 1   M           C           X       I
// 2   MM          CC          XX      II
// 3   MMM         CCC         XXX     III
// 4               CD          XL      IV
// 5               D           L       V
// 6               DC          LX      VI
// 7               DCC         LXX     VII
// 8               DCCC        LXXX    VIII
// 9               CM          XC      IX

fn main() {
    // let mut romans: Vec<&str> = vec![
    //     "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XX", "XXX", "XL", "L", "LX",
    //     "LXX", "LXXX", "XC", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM", "M", "MM",
    //     "MMM",
    // ];
    // let mut romans: [&str; 30] = [
    //     "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XX", "XXX", "XL", "L", "LX",
    //     "LXX", "LXXX", "XC", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM", "M", "MM",
    //     "MMM",
    // ];
    let roman: &str = "MMCDXXI";
    let decimal: u32 = 0;
    let decimal = match roman {
        "MMM" => 3000,
        _ => 0,
    };
    romans.reverse();
    println!("{:#?}", romans);
}
