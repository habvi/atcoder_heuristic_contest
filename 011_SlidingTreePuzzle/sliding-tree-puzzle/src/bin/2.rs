// random move test
use std::io::stdin;
use std::mem::swap;
use std::str::FromStr;
use std::fmt;
use std::cmp::{max, Reverse};
use rand::Rng;

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
                let now: usize = y * g.n + x;
                let nxt: usize = ny * g.n + nx;
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
    assert!(g.gy < g.n && g.gx < g.n, "gy, gx is out of fields");
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

fn move_to_dir(g: &mut Info, dir: &str) -> () {
    add_dir_to_route(g, dir);
    swap_tiles(g, dir);
    add_route_to_cand_ans(g);
}

fn move_1(g: &mut Info) -> () {
    // random t turn
    let dir: Vec<&str> = vec!["U", "D", "L", "R"];
    while g.route.len() < g.t {
        let rdm = rand::thread_rng().gen_range(0, 4);

        let mut ny: i32 = g.gy as i32;
        let mut nx: i32 = g.gx as i32;
        match dir[rdm] {
            "U" => ny -= 1,
            "D" => ny += 1,
            "L" => nx -= 1,
            "R" => nx += 1,
            _ => unreachable!()
        }
        if 0 <= ny && ny < g.n as i32 && 0 <= nx && nx < g.n as i32 {
            move_to_dir(g, dir[rdm]);
        }
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
    eprintln!("{}", g);

    // move
    move_1(&mut g);

    // output
    g.cand_ans.sort_by_key(|x| Reverse(x.0));
    match g.cand_ans.get(0) {
        Some((score, group_size, ans)) => {
            eprintln!("{} {} {} {}", score, group_size, g.route.len(), g.n);
            println!("{}", ans);
        },
        None => println!(),
    }
}
