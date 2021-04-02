#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::{HashMap, HashSet};

const MULTI_TEST: bool = true;

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let n = read.next::<usize>();
    let mut B = read.next_vec::<i64>(n);

    let mut off: i64 = 0;
    let mut sums = HashMap::<i64, u32>::new();
    sums.insert(0, 1);

    let modulus = 1_000_000_007;
    let mut answer: u64 = 1;
    let mut nsums = 1;

    for b in B {
        off += b;
        let nsums_not_null = nsums - sums.get(&-(off - b)).unwrap_or(&0);
        let nsums_equal_to_b = sums.entry(b - off).or_insert(0);

        // Every old sum can be converted to a sum to b.
        *nsums_equal_to_b = nsums % modulus;

        nsums += nsums_not_null + modulus;
        nsums %= modulus;
    }

    println!("{}", nsums);
}

pub fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let mut read = Reader::new(stdin.lock());
    let mut write = std::io::BufWriter::new(stdout.lock());

    let t = if MULTI_TEST { read.next::<u32>() } else { 1 };

    for _ in 0..t {
        solve(&mut read, &mut write);
    }
}

pub struct Reader<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}

impl<B: std::io::BufRead> Reader<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }

            // First, clear the buffer. We want to read new data now.
            self.buf_str.clear();

            // Read the next line.
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed to read the next line.");

            // Split the line into tokens and buffer them.
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                let split_slice = slice.split_whitespace();
                std::mem::transmute(split_slice)
            }
        }
    }

    pub fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.next());
        }

        v
    }

    pub fn next_pair<T: std::str::FromStr>(&mut self) -> (T, T) {
        let first = self.next();
        let second = self.next();

        (first, second)
    }
}
