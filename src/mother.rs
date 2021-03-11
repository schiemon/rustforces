use std::io::{BufRead, BufReader, Stdin};

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

pub fn solve(r: &mut BufReader<Stdin>) {}

pub fn main() {
    const MULTI_TEST: bool = false;
    let r = &mut BufReader::new(std::io::stdin());

    let t = if MULTI_TEST { read::<u32>(r) } else { 1 };

    for _ in 0..t {
        solve(r);
    }
}
