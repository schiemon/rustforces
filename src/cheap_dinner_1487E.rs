use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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

fn read_forbidden_pairs(r: &mut BufReader<Stdin>, inverted: bool) -> HashSet<(usize, usize)> {
    let m = read::<usize>(r);
    let mut forbidden_xy_or_yx = HashSet::<(usize, usize)>::with_capacity(m);
    for _ in 0..m {
        let (x, y): (usize, usize) = read_pair(r);
        if inverted {
            forbidden_xy_or_yx.insert((y - 1, x - 1));
        } else {
            forbidden_xy_or_yx.insert((x - 1, y - 1));
        }
    }

    forbidden_xy_or_yx
}

fn rank(
    X: Vec<(usize, u32)>,
    Y: Vec<(usize, u32)>,
    forbidden_XY: HashSet<(usize, usize)>,
) -> Vec<(usize, u32)> {
    if Y.len() == 0 {
        return vec![];
    }

    let mut ranked = Vec::with_capacity(X.len());
    let mut forbidden_XY = forbidden_XY;
    for (x, c_x) in X.into_iter() {
        for (y, c_y) in Y.iter() {
            let p = &(x, y.to_owned());
            if !forbidden_XY.contains(p) {
                ranked.push((x, c_x + c_y));
                break;
            }
        }
    }

    ranked.sort_unstable_by(|(i, x), (j, y)| x.cmp(&y));
    ranked
}

pub fn solve(r: &mut BufReader<Stdin>) {
    let (a, b, c, d) = read_quadruple::<usize>(r);

    // eprintln!("(a, b, c, d) = ({:?}, {:?}, {:?}, {:?})", a, b, c, d);

    let (mut A, mut B, mut C, mut D) = (
        read_vec::<u32>(r, a)
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, u32)>>(),
        read_vec::<u32>(r, b)
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, u32)>>(),
        read_vec::<u32>(r, c)
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, u32)>>(),
        read_vec::<u32>(r, d)
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, u32)>>(),
    );

    // eprintln!("(A, B, C, D) = ({:?}, {:?}, {:?}, {:?})", A, B, C, D);

    let forbidden_BA = read_forbidden_pairs(r, true);
    let forbidden_BC = read_forbidden_pairs(r, false);
    let forbidden_CD = read_forbidden_pairs(r, false);

    /* eprintln!(
        "(f_BA, f_BC, f_CD) = ({:?}, {:?}, {:?})",
        forbidden_BA, forbidden_BC, forbidden_CD
    ); */

    let cmp_cost: fn(&(usize, u32), &(usize, u32)) -> Ordering = |(i, x), (j, y)| x.cmp(&y);

    A.sort_by(cmp_cost);
    B.sort_by(cmp_cost);
    C.sort_by(cmp_cost);
    D.sort_by(cmp_cost);

    // eprintln!("(A', B', C', D') = ({:?}, {:?}, {:?}, {:?})", A, B, C, D);

    let r_BA = rank(B, A, forbidden_BA);
    let r_CD = rank(C, D, forbidden_CD);

    let mut ok = false;
    let mut answer: u64 = u64::MAX;

    // eprintln!("{:?} {:?}", r_BA, r_CD);

    for (x, c_x) in r_BA.into_iter() {
        for (y, c_y) in r_CD.iter() {
            if !forbidden_BC.contains(&(x, y.to_owned())) {
                ok = true;
                answer = answer.min(c_x as u64 + c_y.to_owned() as u64);
                break;
            }
        }
    }

    if ok {
        println!("{}", answer)
    } else {
        println!("{}", -1)
    }
}
pub fn main() {
    solve(&mut BufReader::new(std::io::stdin()));
}
