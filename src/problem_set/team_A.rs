use std::io::Write;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();
    let mut lines = vec![];
    for i in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let num:Vec<u64> = input.trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        lines.push(num);
    }
    let mut problems_count = 0;
    lines.iter().for_each(|line| {
        let sum = line.iter().fold(0, |acc,x| acc+x);
        if sum >= 2 {
            problems_count += 1;
        }
    });
    
    std::io::stdout().write(format!("{:?}\n",problems_count).as_bytes()).unwrap();
}