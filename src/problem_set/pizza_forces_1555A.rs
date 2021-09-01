use std::io::Write;


fn calc_time(friends:u64)->u64 {
    let mut even_slices = friends;
    if friends % 2 != 0 {
        even_slices = even_slices+1;
    }
    let total = u64::max(even_slices, 6);
    (total* 5)/2
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();
    input.clear();
    let mut tests = vec![];
    for _i in 0..n {
        std::io::stdin().read_line(&mut input).unwrap();
        tests.push(input.trim().parse::<u64>().unwrap());
        input.clear();
    }
    for i in tests {
        let time = calc_time(i);
        std::io::stdout().write(format!("{}\n",time.to_string()).as_bytes()).unwrap();
    }
    
    
}


