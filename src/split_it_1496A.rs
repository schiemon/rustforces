use std::io::{BufRead, BufReader, Read, Stdin};
use std::ops::Range;

type MutRefReader<'a> = &'a mut BufReader<Stdin>;

fn read_vec<F: std::str::FromStr>(r: MutRefReader, n: usize) -> Vec<F>
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

fn read_string(r: MutRefReader) -> String {
    let mut s = String::new();
    r.read_line(&mut s);
    String::from(s.trim())
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

pub fn yes() {
    println!("YES");
}

pub fn no() {
    println!("NO");
}

pub fn rev(s: String) -> String {
    s.chars().rev().collect::<String>()
}

pub fn solve(r: &mut BufReader<Stdin>) {
    let (n, k) = read_pair::<usize>(r);
    let s = read_string(r);

    if k == 0 {
        return yes();
    }

    assert_eq!(s.len(), n);

    let extra = (n + 1) % 2;
    for i in k - 1..n / 2 - extra {
        let left = &s[..i + 1];
        let right = &s[(n - 1 - i)..];
        let right_rev = rev(right.to_owned());
        if left.to_owned() == right_rev {
            return yes();
        }
    }

    no();
}

pub fn main() {
    const MULTI_TEST: bool = true;
    let r = &mut BufReader::new(std::io::stdin());

    let t = if MULTI_TEST { read::<u32>(r) } else { 1 };

    for _ in 0..t {
        solve(r);
    }
}
