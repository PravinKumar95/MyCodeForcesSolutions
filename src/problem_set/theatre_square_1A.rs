use std::io::Write;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num:Vec<u64> = input.trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
    let m = num[0];
    let n = num[1];
    let a = num[2];
    let rowSize = m/a + if m%a > 0 {1} else {0};
    let colSize = n/a + if n%a > 0 {1} else {0};
    std::io::stdout().write(format!("{:?}\n",rowSize*colSize).as_bytes()).unwrap();
}