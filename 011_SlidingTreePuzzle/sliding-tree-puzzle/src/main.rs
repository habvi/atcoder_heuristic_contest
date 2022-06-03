use std::io::stdin;
use std::str::FromStr;
use std::fmt;
use std::mem::swap;
use std::cmp::{max, Reverse};
use rand::Rng;

const DIR: &[&str] = &["U", "D", "L", "R"];

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
    n: usize,
    t: usize,
    route: Vec<String>,
    cand_ans: Vec<(usize, usize, String)>,
    tiles: Vec<Vec<usize>>,
    #[allow(dead_code)]
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

struct UnionFind {
    parent: Vec<usize>,
    v_size: Vec<usize>,
    edge: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            v_size: vec![1; n],
            edge: vec![0; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.root(x);
        y = self.root(y);
        if x == y {
            self.edge[x] += 1;
            return false;
        }
        if self.v_size[x] < self.v_size[y] {
            swap(&mut x, &mut y);
        }
        self.v_size[x] += self.v_size[y];
        self.parent[y] = x;
        self.edge[x] += self.edge[y] + 1;
        true
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn group_size(&mut self, x: usize) -> usize {
        let tmp_root: usize = self.root(x);
        self.v_size[tmp_root]
    }
}

fn to_1dem(g: &mut Info, y: usize, x: usize) -> usize {
    y * g.n + x
}

#[allow(dead_code)]
fn to_2dem(g: &mut Info, z: usize) -> (usize, usize) {
    (z / g.n, z % g.n)
}

fn calc_score(g: &mut Info) -> (usize, usize) {
    let mut uf: UnionFind = UnionFind::new(g.n * g.n);
    let dxy: Vec<(usize, usize)> = vec![(0, 1), (1, 0)];
    for y in 0..g.n - 1 {
        for x in 0..g.n {
            for (dy, dx) in &dxy {
                let ny: usize = y + dy;
                let nx: usize = x + dx;
                if !(ny < g.n && nx < g.n) {
                    continue;
                }
                let now: usize = to_1dem(g, y, x);
                let nxt: usize = to_1dem(g, ny, nx);
                // unite: right, under
                if *dy == 0 {
                    if g.tiles[y][x] >> 2 & 1 == 1 && g.tiles[ny][nx] & 1 == 1 {
                        if !uf.is_same(now, nxt) {
                            uf.unite(now, nxt);
                        }
                    }
                } else {
                    if g.tiles[y][x] >> 3 & 1 == 1 && g.tiles[ny][nx] >> 1 & 1 == 1 {
                        if !uf.is_same(now, nxt) {
                            uf.unite(now, nxt);
                        }
                    }
                }
            }
        }
    }
    let mut max_score: usize = 0;
    let mut max_group_size: usize = 0;
    for i in 0..g.n * g.n {
        let root: usize = uf.root(i);
        let size: usize = uf.group_size(root);
        if i == root && uf.edge[root] + 1 == size {
            let score = if size == g.n * g.n - 1 {
                (500_000. * (2. - g.route.len() as f64 / g.t as f64)).round()
            } else {
                (500_000. * size as f64 / (g.n * g.n - 1) as f64).round()
            } as usize;
            max_score = max(max_score, score);
            max_group_size = max(max_group_size, size);
        }
    }
    (max_score, max_group_size)
}

fn check_range(g: &mut Info, dir: &str) -> bool {
    let mut ny: i32 = g.gy as i32;
    let mut nx: i32 = g.gx as i32;
    match dir {
        "U" => ny -= 1,
        "D" => ny += 1,
        "L" => nx -= 1,
        "R" => nx += 1,
        _ => unreachable!()
    }
    if 0 <= ny && ny < g.n as i32 && 0 <= nx && nx < g.n as i32 {
        return true;
    }
    false
}

fn add_dir_to_route(g: &mut Info, dir: &str) -> () {
    g.route.push(dir.to_string());
}

fn swap_tiles(g: &mut Info, dir: &str) -> () {
    let py: usize = g.gy;
    let px: usize = g.gx;
    match dir {
        "U" => g.gy -= 1,
        "D" => g.gy += 1,
        "L" => g.gx -= 1,
        "R" => g.gx += 1,
        _ => unreachable!()
    }
    let tmp1: usize = g.tiles[py][px];
    let tmp2: usize = g.tiles[g.gy][g.gx];
    g.tiles[py][px] = tmp2;
    g.tiles[g.gy][g.gx] = tmp1;
}

fn add_route_to_cand_ans(g: &mut Info) -> () {
    // just in case
    while g.route.len() > g.t {
        g.route.pop();
    }
    let output: String = g.route.iter().map(|c| c.trim()).collect::<Vec<_>>().join("");

    let (score, group_size): (usize, usize) = calc_score(g);
    g.cand_ans.push((score, group_size, output));
}

fn move_to_dir(g: &mut Info, dir: &str) -> bool {
    if !check_range(g, dir) {
        return false;
    }
    assert!(g.gy < g.n && g.gx < g.n, "gy, gx is out of fields");
    add_dir_to_route(g, dir);
    swap_tiles(g, dir);
    add_route_to_cand_ans(g);
    true
}

// struct Coordinate { y: usize, x: usize }

// bring the target tile:(ty, tx) to (y, x)
#[allow(dead_code)]
#[allow(unused_variables)]
fn bring(g: &mut Info, y: usize, x: usize, ty: usize, tx: usize) -> () {

}

fn ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

fn move_pattern(g: &mut Info) -> () {
    // example
    // move_to_dir(g, "R");
    // move_to_dir(g, "D");
    // move_to_dir(g, "R");
    // move_to_dir(g, "D");
    // move_to_dir(g, "R");
    // move_to_dir(g, "D");
    // move_to_dir(g, "L");
    // move_to_dir(g, "D");

    // make priority for each places
    let mut cand_tile: Vec<Vec<usize>> = vec![Vec::new(); g.n * g.n];
    // (0, 0)
    for &i in [12, 13, 14, 15].iter() {
        cand_tile[0].push(i);
    }
    // (0, 1 ~ n-1)
    for x in 1..g.n - 1 {
        let z: usize = to_1dem(g, 0, x);
        if x % 2 == 0 {
            cand_tile[z].push(12);
            cand_tile[z].push(8);
        } else {
            cand_tile[z].push(9);
            cand_tile[z].push(1);
        }
    }
    // (1, 0)
    for &i in [10, 11, 14, 15, 2].iter() {
        let z: usize = to_1dem(g, 1, 0);
        cand_tile[z].push(i);
    }
    // (1, 1 ~ n-1)
    for x in 1..g.n - 1 {
        let z: usize = to_1dem(g, 1, x);
        if x % 2 == 0 {
            cand_tile[z].push(3);
            cand_tile[z].push(1);
        } else {
            cand_tile[z].push(6);
            cand_tile[z].push(2);
        }
    }

    let sum57: usize = g.tile_num[5] + g.tile_num[7];
    let row: usize = ceil(sum57, g.n - 2);
    let horizontal: usize = row;
    let vertical: usize = g.n - 3 - row;
    eprintln!("horizontal: {}, vertical: {}", horizontal, vertical);
    // horizontal
    // (2 ~ 2+horizontal, 0)
    for y in 2..2 + horizontal {
        for &i in [14, 15].iter() {
            let z: usize = to_1dem(g, y, 0);
            cand_tile[z].push(i);
        }
    }
    // (2 ~ 2+horizontal, 1 ~ n-1)
    for y in 2..2 + horizontal {
        for x in 1..g.n - 1 {
            let z: usize = to_1dem(g, y, x);
            for &i in [5, 7, 1].iter() {
                cand_tile[z].push(i);
            }
        }
    }

    // vertical
    let start: usize = 2 + horizontal;
    // (start, 0)
    for &i in [14, 15, 2].iter() {
        let z: usize = to_1dem(g, start, 0);
        cand_tile[z].push(i);
    }
    // (start, 1 ~ n-1)
    for x in 1..g.n - 1 {
        let z: usize = to_1dem(g, start, x);
        for &i in [13, 5, 7, 1].iter() {
            cand_tile[z].push(i);
        }
    }
    // (start+1 ~ n-1, 0)
    for y in start + 1..g.n - 1 {
        let z: usize = to_1dem(g, y, 0);
        for &i in [11, 10, 15, 3, 2].iter() {
            cand_tile[z].push(i);
        }
    }
    // (start+1 ~ n-1, 1 ~ n-1)
    for y in start + 1..g.n - 1 {
        for x in 1..g.n - 1 {
            let z: usize = to_1dem(g, y, x);
            for &i in [10, 11, 15, 2].iter() {
                cand_tile[z].push(i);
            }
        }
    }
    eprintln!("{:?}", cand_tile);
}

fn move_random(g: &mut Info) -> () {
    while g.route.len() < g.t {
        let rdm = rand::thread_rng().gen_range(0, 4);
        move_to_dir(g, DIR[rdm]);
    }
}

fn main() {
    // input
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
    // eprintln!("{}", g);

    // move
    move_pattern(&mut g);
    move_random(&mut g);

    // output
    g.cand_ans.sort_by_key(|x| Reverse(x.0));
    // eprintln!("{}", g);
    match g.cand_ans.get(0) {
        Some((score, group_size, ans)) => {
            eprintln!("{} {} {} {}", score, group_size, g.route.len(), g.n);
            // eprintln!("{}", ans);
            println!("{}", ans);
        },
        None => println!(),
    }
    // eprintln!("{}", g.cand_ans.len());
}
