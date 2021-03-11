use std::io::{BufRead, BufReader, Stdin};

fn read_vec<F: std::str::FromStr>(r: &mut BufReader<Stdin>, n: usize) -> Vec<F>
where
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut nums_str = String::new();
    r.read_line(&mut nums_str).unwrap();
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
const MOD: u64 = 1000000007;
const BITS: usize = 62;

pub fn solve(r: &mut BufReader<Stdin>, pow_of_two: [u64; BITS]) {
    let n = read::<usize>(r);
    let X = read_vec::<u64>(r, n)
        .into_iter()
        .collect::<Vec<u64>>() ;

    let mut number_bit_owners: [u64; BITS] = [0; BITS];

    for x in X.iter() {
        for b in 0..BITS {
            number_bit_owners[b] += is_set(*x, b);
        }
    }

    for b in 0..BITS {
        number_bit_owners[b] %= MOD;
    }

    // eprintln!("P: {:?}", P);
    // eprintln!("B: {:?}", B);

    #[inline]
    fn is_not_set(x: u64, b: usize) -> u64 {
        1 - is_set(x, b)
    }

    #[inline]
    fn is_set(x: u64, b: usize) -> u64 {
        (x >> b) & 1
    }

    let f = |j: usize| -> u64 {
        (0..BITS)
            .map(|b| (pow_of_two[b] * (n as u64 - is_not_set(X[j], b) * (n as u64 - number_bit_owners[b])) % MOD))
            .sum::<u64>()
            % MOD
    };

    let g = |j: usize| -> u64 {
        (0..BITS)
            .map(|b| (pow_of_two[b] * is_set(X[j], b) * number_bit_owners[b]) % MOD)
            .sum::<u64>()
            % MOD
    };

    println!("{}", (0..n).map(|j| f(j) * g(j) % MOD).sum::<u64>() % MOD)
}
pub fn main() {
    const MULTI_TEST: bool = true;
    let r = &mut BufReader::new(std::io::stdin());

    let t = if MULTI_TEST { read::<u32>(r) } else { 1 };

    // Compute P (power of two in modulus MOD)
    let mut pow_of_two: [u64; BITS] = [1; BITS];
    for i in 1..BITS {
        pow_of_two[i] = 2 * pow_of_two[i - 1] % MOD;
    }

    for _ in 0..t {
        solve(r, pow_of_two);
    }
}
