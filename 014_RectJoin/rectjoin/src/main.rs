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

type Point = (i32, i32);

#[derive(Debug)]
pub struct Input {
    n: usize,
    m: usize,
    xy: Vec<Point>,
}

type Output = Vec<[Point; 4]>;

fn weight((x, y): Point, n: usize) -> i32 {
    let dx = x - n as i32 / 2;
    let dy = y - n as i32 / 2;
    dx * dx + dy * dy + 1
}

const DXY: [(i32, i32); 8] = [
	(1, 0),
	(1, 1),
	(0, 1),
	(-1, 1),
	(-1, 0),
	(-1, -1),
	(0, -1),
	(1, -1)
];

#[derive(Debug)]
struct State {
	has_point: Vec<Vec<bool>>,
	used: Vec<Vec<[bool; 8]>>,
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

impl State {
	fn new(input: &Input) -> Self {
		let mut has_point = mat![false; input.n; input.n];
		let used = mat![[false; 8]; input.n; input.n];
		for i in 0..input.xy.len() {
			has_point[input.xy[i].0 as usize][input.xy[i].1 as usize] = true;
		}
		Self {
			has_point,
			used,
		}
	}
	fn check_move(&self, rect: [Point; 4]) -> String {
		if let Some(i) = (1..4).find(|&i| !self.has_point[rect[i].0 as usize][rect[i].1 as usize]) {
			return format!("({}, {}) does not contain a dot", rect[i].0, rect[i].1);
		} else if self.has_point[rect[0].0 as usize][rect[0].1 as usize] {
			return format!("({}, {}) already contains a dot", rect[0].0, rect[0].1);
		} else {
			let dx01 = rect[1].0 - rect[0].0;
			let dy01 = rect[1].1 - rect[0].1;
			let dx03 = rect[3].0 - rect[0].0;
			let dy03 = rect[3].1 - rect[0].1;
			if dx01 * dx03 + dy01 * dy03 != 0 {
				return "Illegal rectangle".to_owned();
			} else if dx01 != 0 && dy01 != 0 && dx01.abs() != dy01.abs() {
				return "Illegal rectangle".to_owned();
			} else if (rect[1].0 + dx03, rect[1].1 + dy03) != rect[2] {
				return "Illegal rectangle".to_owned();
			} else {
				for i in 0..4 {
					let (mut x, mut y) = rect[i];
					let (tx, ty) = rect[(i + 1) % 4];
					let dx = (tx - x).signum();
					let dy = (ty - y).signum();
					let dir = (0..8).find(|&dir| DXY[dir] == (dx, dy)).unwrap();
					while (x, y) != (tx, ty) {
						if (x, y) != rect[i] && self.has_point[x as usize][y as usize] {
							return format!("There is an obstacle at ({}, {})", x, y);
						}
						if self.used[x as usize][y as usize][dir] {
							return "Overlapped rectangles".to_owned();
						}
						x += dx;
						y += dy;
						if self.used[x as usize][y as usize][dir ^ 4] {
							return "Overlapped rectangles".to_owned();
						}
					}
				}
			}
		}
		String::new()
	}
	fn apply_move(&mut self, rect: [Point; 4]) {
		self.has_point[rect[0].0 as usize][rect[0].1 as usize] = true;
		for i in 0..4 {
			let (mut x, mut y) = rect[i];
			let (tx, ty) = rect[(i + 1) % 4];
			let dx = (tx - x).signum();
			let dy = (ty - y).signum();
			let dir = (0..8).find(|&dir| DXY[dir] == (dx, dy)).unwrap();
			while (x, y) != (tx, ty) {
				self.used[x as usize][y as usize][dir] = true;
				x += dx;
				y += dy;
				self.used[x as usize][y as usize][dir ^ 4] = true;
			}
		}
	}
}

fn compute_score(input: &Input, out: &[[Point; 4]]) -> (i64, String, State) {
	let mut state = State::new(input);
	for t in 0..out.len() {
		let err = state.check_move(out[t]);
		if err.len() > 0 {
			return (0, format!("{} (turn: {})", err, t), state);
		}
		state.apply_move(out[t]);
	}
	let mut num = 0;
	for &p in &input.xy {
		num += weight(p, input.n);
	}
	for rect in out {
		num += weight(rect[0], input.n);
	}
	let mut den = 0;
	for i in 0..input.n {
		for j in 0..input.n {
			den += weight((i as i32, j as i32), input.n);
		}
	}
	let score = (1e6 * (input.n * input.n) as f64 / input.m as f64 * num as f64 / den as f64).round() as i64;
	(score, String::new(), state)
}

fn solve(output: &mut Output) -> () {
    let xy = [(11, 11), (12, 11), (12, 12), (11, 12)];
    output.push(xy);
}

#[allow(dead_code)]
fn print_output(output: Output) -> () {
    let n = output.len();
    println!("{}", n);
    for ps in output {
        for (x, y) in &ps {
            print!("{} {} ", x, y);
        }
        println!("");
    }
}

fn main() {
    let (n, m): (usize, usize) = input_tuple();
    let mut input = Input {
        n: n,
        m: m,
        xy: Vec::new(),
    };
    for _ in 0..m {
        let point: Point= input_tuple();
        input.xy.push(point);
    }
    eprintln!("{:?}\n", input);

    let mut output: Output = Vec::new();
    solve(&mut output);

    #[allow(unused_variables)]
    let (score, string, state) = compute_score(&input, &output);
    eprintln!("score : {}\n", score);

    // print_output(output);
    println!("0");
}
