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
/*
pub fn longest_cycle_in_open(A: &[i32], B: &[i32], C: &[i32], count_first: bool) -> u32 {
    /*println!("{}", count_first);
    println!("C: {:?}", C);
    println!("A: {:?}", A);
    println!("B: {:?}", B);*/

    assert_eq!(A.len(), B.len());
    assert_eq!(B.len(), C.len());

    let n = A.len();

    if n < 2 {
        return 0;
    }

    let mut curr_cycle = if count_first { 1 } else { 0 } + C[0] as i32;
    let mut answer = if count_first { curr_cycle } else { 0 };
    for i in 1..n {
        assert!(0 <= A[i]);
        assert!(A[i] < B[i]);
        assert!(B[i] <= C[i - 1]);

        let top_to_exclude = A[i] - 1;
        let exc = B[i] - 1 - A[i];
        // println!("> total exclude: {} {}", exc, answer);
        // println!("> curr_cycle = {}, answer = {}", curr_cycle, answer);
        curr_cycle = curr_cycle - if i > 1 { exc } else { 0 } + C[i];
        answer = answer.max(curr_cycle);
        // println!("< curr_cycle = {}, answer = {}", curr_cycle, answer);

    }

    answer as u32
}
*/
pub fn solve(r: &mut BufReader<Stdin>) {
    let n = read::<usize>(r);
    let mut C = read_vec::<u32>(r, n);
    let mut A = read_vec::<i32>(r, n);
    let mut B = read_vec::<i32>(r, n);

    assert_eq!(A[0], -1);
    assert_eq!(B[0], -1);

    // println!("{:?} {:?} {:?}", A, B, C);

    let mut answer = 0;
    // The first chain cannot be connected to another chain to the left.
    let mut len_cycle_before : u64 = 0; // artifically add two node and pretend that we are already connected.
    for i in 1..n {
        let a = A[i].min(B[i]);
        let b = A[i].max(B[i]);
        // interior := |(a, b)|
        let interior = b - a - 1;
        // println!("[{}]: interior: {}", i, interior);
        
        assert!(interior >= -1);
        if interior == -1 {
            // We cannot create a cycle because of this bottleneck.
            len_cycle_before = (C[i] + 1) as u64;
        } else {
            // exterior := |[1, a] + [b, C[i - 1]]|
            let exterior = C[i - 1] - interior as u32;
            // -2 because we do not want to overcount the beginnings and ends.
            if i == 1 {
                len_cycle_before = (interior as u64 + 2 + C[i] as u64) as u64
            } else {
                len_cycle_before =(interior as u64 + 2).max(len_cycle_before - C[i - 1] as u64  + exterior as u64) + C[i] as u64;
            }
        }

        // println!("[{}]: len_cycle_before: {}", i, len_cycle_before);

        answer = answer.max(len_cycle_before)
        /*
     ......-1  _______1
            | /       |
            |/        |
        A[i - 1]      |
            |         |
            |         |
        B[i - 1]      |
            |\        |
            | \_______|
     ...C[i - 1]     C[i]
        */

        // Either EBCF = EF + AD - BC + 2 or EBA...DCFE = ... + AB + CD + EF
    }

    println!("{}", answer);
}

pub fn main() {
    const MULTI_TEST: bool = true;
    let r = &mut BufReader::new(std::io::stdin());

    let t = if MULTI_TEST { read::<u32>(r) } else { 1 };

    for _ in 0..t {
        solve(r);
    }
}
