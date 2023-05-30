use std::env;

const fn test(n: u64) -> u64 {
    u64::MAX + n
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n:u64 = args[1].parse::<u64>().unwrap();
    let r:u64 = test(n);
    println!("{}", r);
}
