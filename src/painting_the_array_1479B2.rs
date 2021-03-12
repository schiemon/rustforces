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

fn contract(a: Vec<u32>) -> Vec<(u32, u32)> {
    // Contracts a into segments consisting of same elements each.
    // We are doing this by noting the length of each segment.

    if a.len() == 0 {
        return Vec::new();
    }

    let mut last = a[0];
    let mut curr_length = 1;
    let mut b = Vec::with_capacity(a.len());
    // println!("{:?}", a);
    for x in a.into_iter().skip(1) {
        // println!("> {} {}", x, curr_length);
        if x == last {
            curr_length += 1;
        } else {
            b.push((last, curr_length));
            curr_length = 1;
            last = x;
        }
    }

    b.push((last, curr_length));
    b
}


pub fn solve(r: &mut BufReader<Stdin>) {
    let n = read::<usize>(r);
    let a = read_vec::<u32>(r, n);

    let b = contract(a);

    // println!("{:?}", b);

    let mut c = Vec::<u32>::with_capacity(b.len());
    for (x, l) in b.iter() {
        if *l >= 2 {
            if !c.is_empty() {
                if c[c.len() - 1] != *x {
                    c.push(*x);
                }
            } else {
                c.push(*x);
            }
        }
    }

    println!("{}", b.len() + c.len());
}

pub fn main() {
    const MULTI_TEST: bool = false;
    let r = &mut BufReader::new(std::io::stdin());

    let t = if MULTI_TEST { read::<u32>(r) } else { 1 };

    for _ in 0..t {
        solve(r);
    }
}
