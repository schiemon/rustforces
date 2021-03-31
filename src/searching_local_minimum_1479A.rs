use std::{
    io::{BufRead, BufReader, BufWriter, Stdin, Write},
    process::exit,
};

fn read_vec<F: std::str::FromStr>(r: &mut impl BufRead, n: usize) -> Vec<F>
where
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut nums_str = String::new();
    r.read_line(&mut nums_str).expect("Cannot read line.");
    let nums_vec: Vec<F> = nums_str
        .split_whitespace()
        .map(|x| x.parse::<F>().unwrap())
        .collect();

    assert_eq!(nums_vec.len(), n);

    nums_vec
}

fn read<F>(r: &mut impl BufRead) -> F
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_vec(r, 1)[0]
}

fn read_pair<F>(r: &mut impl BufRead) -> (F, F)
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    match read_vec(r, 2).as_slice() {
        [a, b, ..] => (*a, *b), // We can only get references. Copy this.
        [..] => unreachable!("Called read(u32) did not asserted the length of the input."),
    }
}

fn read_quadruple<F>(r: &mut impl BufRead) -> (F, F, F, F)
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    match read_vec(r, 4).as_slice() {
        [a, b, c, d, ..] => (*a, *b, *c, *d), // We can only get references. Copy this.
        [..] => unreachable!("Called read(u32) did not asserted the length of the input."),
    }
}

fn query(r: &mut impl BufRead, w: &mut impl Write, i: usize) -> u32 {
    write!(w, "? {}\n", i).unwrap();
    w.flush().unwrap();
    read(r)
}

fn answer(w: &mut impl Write, i: usize) {
    write!(w, "! {} \n", i).unwrap();
    w.flush().unwrap();
    exit(0);
}

fn solve(r: &mut BufReader<Stdin>, w: &mut impl Write) {
    let INF = std::primitive::u32::MAX;

    let n: usize = read(r);

    let mut lo = 1_usize;
    let mut hi = n;

    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        // dbg!(&mid);
        let prev = if mid > 1 { query(r, w, mid - 1) } else { INF };

        let curr = query(r, w, mid);

        let next = if mid < n { query(r, w, mid + 1) } else { INF };

        // Check if local minimum.
        let mut local_min = false;
        let mut descending = false;
        if prev > curr && curr < next {
            local_min = true;
        } else if prev > curr && curr > next {
            descending = true;
        }

        if local_min {
            answer(w, mid);
            return;
        } else if descending {
            // descending
            /*
                INF ... 3 2 {1 ... INF} <- l.m. is here
            */
            lo = mid + 1;
        } else {
            // ascending
            /*
                l.m. is here -> {INF ... 1} 2 3 ... INF
            */
            hi = mid - 1;
        }
    }

    let lo_val = query(r, w, lo);
    let hi_val = query(r, w, hi);

    let k = if lo_val < hi_val { lo } else { hi };

    answer(w, k);
    w.flush().unwrap();
}

pub fn main() {
    const MULTI_TEST: bool = false;
    let r = &mut BufReader::new(std::io::stdin());
    let w = &mut BufWriter::new(std::io::stdout());

    let t = if MULTI_TEST { read(r) } else { 1 };

    for _ in 0..t {
        solve(r, w);
    }
}
