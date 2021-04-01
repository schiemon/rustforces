#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::mem::swap;

fn is_pairable(A: &[u32], B: &[u32]) -> bool {
    assert_eq!(A.len(), B.len());

    for i in 0..A.len() {
        if !(A[i] < B[i]) {
            return false;
        }
    }

    return true;
}

const MULTI_TEST: bool = true;

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let n = read.next::<usize>();
    let mut B: Vec<u32> = Vec::with_capacity(n);

    for i in 0..n {
        B.push(read.next::<u32>());
    }

    let mut A = Vec::with_capacity(n);

    let mut last = 0;
    let mut cs = Vec::with_capacity(1 + n + 1);

    // Get complement.
    cs.push(0);
    cs.extend(B.iter().cloned());
    cs.push(2 * n as u32 + 1);
    for i in 0..n + 1 {
        A.extend(cs[i] + 1..cs[i + 1])
    }

    let mut limits = Vec::new();
    for _ in 0..2 {
        // Get max value for x.
        let mut lo = 0;
        let mut hi = n;

        while lo < hi {
            let x = lo + (hi - lo + 2 - 1) / 2;

            if is_pairable(&B[..x], &A[n - x..]) {
                lo = x;
            } else {
                hi = x - 1;
            }
        }

        limits.push(lo); // Yeah, push them!
        swap(&mut B, &mut A);
    }

    let hi = limits[0];
    let lo = n - limits[1];

    assert!(hi >= lo);

    println!("{}", hi - lo + 1);
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

            // Split the line into token and buffer them.
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
