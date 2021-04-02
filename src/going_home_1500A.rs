#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

const MULTI_TEST: bool = false;

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let n = read.next::<usize>();
    let mut A: Vec<u32> = Vec::with_capacity(n);

    for i in 0..n {
        A.push(read.next::<u32>());
    }

    // println!("{}", n);
    assert!(n >= 4);

    let mut sums = HashMap::<u32, (usize, [(usize, usize); 4])>::new();
    for x in 0..n {
        for y in x + 1..n {
            let sum = A[x] + A[y];
            let pairs_for_sum = sums.entry(sum).or_insert((0, [(n, n); 4]));

            pairs_for_sum.1[pairs_for_sum.0] = (x, y);
            pairs_for_sum.0 += 1;

            let mut indices = Vec::<usize>::new();

            for (a, b) in pairs_for_sum.1.iter() {
                if *a == n {
                    break;
                }

                indices.push(*a);
                indices.push(*b);
            }

            let nindices = indices.len();

            for i in 0..nindices {
                let x = indices[i];
                for j in i + 1..nindices {
                    let y = indices[j];
                    for k in j + 1..nindices {
                        let z = indices[k];
                        for l in k + 1..nindices {
                            let w = indices[l];

                            let mut xyzw = [x, y, z, w];
                            xyzw.sort();
                            let mut ok = true;
                            for i in 0..3 {
                                if xyzw[i] == xyzw[i + 1] {
                                    ok = false;
                                    break;
                                }
                            }

                            if ok
                                && (A[x] + A[y] == A[z] + A[w]
                                    || A[x] + A[z] == A[y] + A[w]
                                    || A[x] + A[w] == A[y] + A[z])
                            {
                                println!("YES");
                                println!("{} {} {} {} ", x + 1, y + 1, z + 1, w + 1);
                                exit(0);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("NO");
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
