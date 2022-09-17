use std::io::stdin;
use std::str::FromStr;

fn input_tuple<T: FromStr>() -> (T, T) {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    let mut itr = s.split_whitespace();
    (
        itr.next().unwrap().parse().ok().unwrap(),
        itr.next().unwrap().parse().ok().unwrap()
    )
}

#[allow(unused_variables)]
fn main() {
    let (n, m): (usize, usize) = input_tuple();
    for _ in 0..m {
        let (x, y): (usize, usize) = input_tuple();
    }
    println!("0");
}
