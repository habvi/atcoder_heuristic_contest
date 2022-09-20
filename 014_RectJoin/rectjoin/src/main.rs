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

// https://atcoder.jp/contests/intro-heuristics/submissions/14855120
fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
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

impl State {
    fn new(input: &Input) -> Self {
        let mut has_point = vec![vec![false; input.n]; input.n];
        let used = vec![vec![[false; 8]; input.n]; input.n];
        for i in 0..input.m {
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

fn solve(input: &Input, output: &mut Output) -> () {
    let mut state = State::new(input);
    while get_time() < 4.0 {
        let mut max_score = -1;
        let mut max_rect = [(-1, -1); 4];
        for i in 0..input.n {
            for j in 0..input.n {
                if state.has_point[i][j] == true {
                    continue;
                }
                for k in 0..8 {
                    let mut rect = [(-1, -1); 4];
                    // [0]
                    rect[0] = (i as i32, j as i32);
                    // [1] : right
                    let (mut x, mut y) = (i as i32, j as i32);
                    let (dx, dy) = DXY[k];
                    while 0 <= x && x < input.n as i32 && 0 <= y && y < input.n as i32 {
                        if state.has_point[x as usize][y as usize] == true {
                            rect[1] = (x, y);
                            break;
                        }
                        x += dx;
                        y += dy;
                    }
                    if rect[1] == (-1, -1) {
                        continue;
                    }
                    // [3] : left
                    let (mut x, mut y) = (i as i32, j as i32);
                    let (dx, dy) = DXY[(k + 2) % 8];
                    while 0 <= x && x < input.n as i32 && 0 <= y && y < input.n as i32 {
                        if state.has_point[x as usize][y as usize] == true {
                            rect[3] = (x, y);
                            break;
                        }
                        x += dx;
                        y += dy;
                    }
                    if rect[3] == (-1, -1) {
                        continue;
                    }
                    // [2]
                    let (dx, dy) = (rect[1].0 - rect[0].0, rect[1].1 - rect[0].1);
                    let (x2, y2) = (rect[3].0 + dx, rect[3].1 + dy);
                    if !(0 <= x2 && x2 < input.n as i32 && 0 <= y2 && y2 < input.n as i32) {
                        continue;
                    }
                    rect[2] = (x2, y2);

                    let err = state.check_move(rect);
                    if err.len() > 0 {
                        continue;
                    }
                    let w = weight(rect[0], input.n);
                    if w > max_score {
                        max_score = w;
                        max_rect = rect;
                    }
                }
            }
        }
        if max_score == -1 {
            break;
        }
        eprintln!("{} {:?}", max_score, max_rect);
        state.apply_move(max_rect);
        output.push(max_rect);
    }
}

fn main() {
    let (n, m): (usize, usize) = input_tuple();
    let mut xy = Vec::new();
    for _ in 0..m {
        let point: Point = input_tuple();
        xy.push(point);
    }
    let input = Input {
        n,
        m,
        xy,
    };
    eprintln!("{:?}\n", input);

    get_time();
    let mut output: Output = Vec::new();
    solve(&input, &mut output);
    eprintln!("\nOutput : {:?}\n", output);

    #[allow(unused_variables)]
    let (score, string, state) = compute_score(&input, &output);
    if string.len() > 0 {
        eprintln!("(main: compute score) Error!! {}\n", string);
        println!("0");
    } else {
        eprintln!("Score : {}\n", score);
        print_output(output);
    }
}
