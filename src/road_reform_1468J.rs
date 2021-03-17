use std::io::{BufRead, BufReader, Stdin};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn read_vec<F: std::str::FromStr>(r: &mut BufReader<Stdin>, n: usize) -> Vec<F>
where
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut nums_str = String::new();
    r.read_line(&mut nums_str);
    let nums_vec: Vec<F> = nums_str
        .split_whitespace()
        .map(|x| x.parse::<F>().unwrap())
        .collect();

    assert_eq!(nums_vec.len(), n);

    nums_vec
}

fn read<F>(r: &mut BufReader<Stdin>) -> F
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_vec::<F>(r, 1)[0]
}

fn read_pair<F>(r: &mut BufReader<Stdin>) -> (F, F)
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    match read_vec::<F>(r, 2).as_slice() {
        [a, b, ..] => (*a, *b), // We can only get references. Copy this.
        [..] => unreachable!("Called read(u32) did not asserted the length of the input."),
    }
}

fn read_triple<F>(r: &mut BufReader<Stdin>) -> (F, F, F)
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    match read_vec::<F>(r, 3).as_slice() {
        [a, b, c, ..] => (*a, *b, *c), // We can only get references. Copy this.
        [..] => unreachable!("Called read(u32) did not asserted the length of the input."),
    }
}

fn read_quadruple<F>(r: &mut BufReader<Stdin>) -> (F, F, F, F)
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    match read_vec::<F>(r, 4).as_slice() {
        [a, b, c, d, ..] => (*a, *b, *c, *d), // We can only get references. Copy this.
        [..] => unreachable!("Called read(u32) did not asserted the length of the input."),
    }
}

type WeightedEdge = (usize, usize, u32);

// (x, cost)
fn mst_prim(n: usize, g: &Vec<Vec<(usize, u32)>>) -> Vec<WeightedEdge> {
    let mut visited = vec![false; n];
    let mut t = Vec::<WeightedEdge>::with_capacity(n - 1);

    // (cost, neighbour)
    let mut pending = BinaryHeap::<(Reverse<u32>, WeightedEdge)>::with_capacity(n);
    pending.push((Reverse(0), (0, 0, 0)));

    while pending.len() > 0 && t.len() != n - 1 {
        let (_, we) = pending.pop().unwrap();
        let x = we.1;
        if visited[x] {
            continue;
        }
        visited[x] = true;
        if x != 0 {
            t.push(we);
        }
        for (y, c) in g[we.1].iter() {
            if !visited[*y] {
                pending.push((Reverse(*c), (x, *y, *c)));
            }
        }

        // eprintln!("@ {}: {:?}", x, pending);
    }

    t
}

fn solve(r: &mut BufReader<Stdin>) {
    let (n, m, k) = read_triple::<usize>(r);

    let mut g = vec![Vec::new(); n];
    let mut mn_diff = std::primitive::u32::MAX;

    for _ in 0..m {
        let (x, y, s) = read_triple::<u32>(r);
        let x = (x - 1) as usize;
        let y = (y - 1) as usize;

        let diff = s as i32 - k as i32;
        let clipped_diff = diff.max(0) as u32;

        g[x].push((y, clipped_diff));
        g[y].push((x, clipped_diff));

        mn_diff = mn_diff.min(diff.abs() as u32);
    }

    let mst = mst_prim(n, &g);
    // eprintln!("{:?}", mst);
    let acc_cost_for_too_fast = mst.iter().map(|(_, _, c)| *c as u64).sum::<u64>();

    if acc_cost_for_too_fast == 0 {
        println!("{}", mn_diff);
    } else {
        println!("{}", acc_cost_for_too_fast);
    }
}

pub fn main() {
    const MULTI_TEST: bool = true;
    let r = &mut BufReader::new(std::io::stdin());

    let t = if MULTI_TEST { read::<u32>(r) } else { 1 };

    for _ in 0..t {
        solve(r);
    }
}
