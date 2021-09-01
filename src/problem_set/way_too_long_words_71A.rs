use std::io::Write;

fn process_text(s:&str) -> String{
    if s.len() > 10 {
        format!("{}{}{}",s.chars().nth(0).unwrap(),s.len()-2,s.chars().nth(s.len()-1).unwrap())
    }else{
        s.into()
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();
    input.clear();
    let mut tests:Vec<String> = vec![];
    for _i in 0..n {
        std::io::stdin().read_line(&mut input).unwrap();
        tests.push(input.trim().into());
        input.clear();
    }
    for i in tests {
        let r = process_text(&i);
        std::io::stdout().write(format!("{}\n",r).as_bytes()).unwrap();
    }
}