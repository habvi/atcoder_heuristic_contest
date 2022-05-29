use std::io::stdin;
use std::str::FromStr;

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

fn move_test(ans: &mut Vec<&str>) -> () {
    ans.push("U");
    ans.push("D");
    ans.push("R");
    ans.push("L");
}

fn main() {
    let nt: Vec<usize> = input_vec();
    let n: usize = nt[0];
    // max times : 2 * n^3
    #[allow(unused_variables)]
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
    eprintln!("start (y, x) : {}, {}", gy, gx);
    eprintln!("tile num : {:?}", tile_num);

    let mut ans: Vec<&str> = Vec::new();

    // move
    move_test(&mut ans);

    // output
    let output: String = ans.iter().map(|c| c.trim()).collect::<Vec<_>>().join("");
    eprintln!("{}", output);
    println!("{}", output)
}
