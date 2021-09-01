use std::io::Write;


fn check_even(c:u32)->String {
    if c > 2 && (c - 2)%2 == 0 {
        format!("{}","YES")
    }
    else {
        format!("{}","NO")
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let c = input.trim().parse::<u32>().unwrap();
    std::io::stdout().write(check_even(c).as_bytes()).unwrap();
}

mod test{
    #[test]
    fn testing() {
        let out = super::check_even(10);
        assert_eq!("YES",out);
    }
}
