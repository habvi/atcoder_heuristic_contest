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

fn eprint_tiles(tiles: &mut Vec<Vec<usize>>) -> () {
    for tile in tiles.iter() {
        for t in tile.iter() {
            eprint!("{} ", t);
        }
        eprintln!();
    }
    eprintln!("----")
}

struct Info {
    ans: Vec<String>,
    tiles: Vec<Vec<usize>>,
    tile_num: Vec<usize>,
    gy: usize,
    gx: usize,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gy, gx : ({}, {})\ntiles : {:?}\n-----",
        self.gx, self.gy, self.tiles)
    }
}

fn move_1(g: &mut Info) -> () {
    // example
    g.ans.push("UUUU".to_string());
    let tmp1: usize = g.tiles[0][0];
    let tmp2: usize = g.tiles[1][0];
    g.tiles[1][0] = tmp1;
    g.tiles[0][0] = tmp2;
    g.tile_num[0] += 0;
    g.gx += 1;
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
    eprint_tiles(&mut tiles);
    eprintln!("tile num : {:?}", tile_num);
    eprintln!("-----");

    let ans: Vec<String> = Vec::new();
    let mut g: Info = Info {
        ans: ans,
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
    while g.ans.len() > t {
        g.ans.pop();
    }
    let output: String = g.ans.iter().map(|c| c.trim()).collect::<Vec<_>>().join("");
    eprintln!("{}", output);
    println!("{}", output)
}
