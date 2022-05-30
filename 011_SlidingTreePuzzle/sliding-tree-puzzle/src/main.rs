use std::io::stdin;
use std::str::FromStr;
use std::fmt;

fn input_t<T: FromStr>() -> T {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok().unwrap();
    s.trim().parse().ok().unwrap()
}

fn input_vec<T: FromStr>() -> Vec<T> {
    input_t::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

struct Info {
    #[allow(dead_code)]
    n: usize,
    t: usize,
    route: Vec<String>,
    cand_ans: Vec<(usize, String)>,
    tiles: Vec<Vec<usize>>,
    tile_num: Vec<usize>,
    gy: usize,
    gx: usize,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "gy, gx : ({}, {})", self.gx, self.gy)?;
        for tile in self.tiles.iter() {
            for t in tile.iter() {
                write!(f, "{} ", t)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "{:?}", self.cand_ans)?;
        write!(f, "----")?;
        Ok(())
    }
}

#[allow(unused_variables)]
fn calc_score(g: &mut Info) -> usize {
    100000
}

fn add_cand_ans(g: &mut Info) -> () {
    while g.route.len() > g.t {
        g.route.pop();
    }
    let output: String = g.route.iter().map(|c| c.trim()).collect::<Vec<_>>().join("");

    let score: usize = calc_score(g);
    g.cand_ans.push((score, output));
}

fn move_1(g: &mut Info) -> () {
    // example
    g.route.push("U".to_string());
    g.route.push("D".to_string());
    let tmp1: usize = g.tiles[0][0];
    let tmp2: usize = g.tiles[1][0];
    g.tiles[1][0] = tmp1;
    g.tiles[0][0] = tmp2;
    g.tile_num[0] += 0;
    g.gx += 1;
    g.cand_ans.push((600, "UUDD".to_string()));
    g.cand_ans.push((800, "UUUUUUUUU".to_string()));
    g.cand_ans.push((100, "RRRLLLL".to_string()));

    // each turn
    add_cand_ans(g);
}

fn main() {
    let nt: Vec<usize> = input_vec();
    let n: usize = nt[0];
    // max times : 2 * n^3
    let t: usize = nt[1];

    let mut tiles: Vec<Vec<usize>> = Vec::new();
    let mut gy: usize = 0;
    let mut gx: usize = 0;

    const TYPES: usize = 16;
    let mut tile_num: Vec<usize> = vec![0; TYPES];

    for y in 0..n {
        let input: String = input_t();
        let mut v: Vec<usize> = Vec::new();
        for (x, a) in input.chars().enumerate() {
            // to 0 ~ 15
            let num: usize;
            if '0' <= a && a <= '9' {
                num = a as usize - 48;
                if num == 0 {
                    gy = y;
                    gx = x;
                }
            } else {
                num = a as usize - 87;
            }
            v.push(num);
            tile_num[num] += 1;
        }
        tiles.push(v);
    }
    assert_eq!(0, tiles[gy][gx]);
    eprintln!("tile num : {:?}", tile_num);
    eprintln!("-----");

    let mut g: Info = Info {
        n: n,
        t: t,
        route: Vec::new(),
        cand_ans: Vec::new(),
        tiles: tiles,
        tile_num: tile_num,
        gy: gy,
        gx: gx,
    };

    eprintln!("{}", g);

    // move
    move_1(&mut g);
    eprintln!("{}", g);

    // output
    g.cand_ans.sort_by_key(|x| std::cmp::Reverse(x.0));
    eprintln!("{}", g);
    eprintln!("{}", g.cand_ans[0].1);

    println!("{}", g.cand_ans[0].1);
}
