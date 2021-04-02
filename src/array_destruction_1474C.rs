#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    process::exit,
};

const MULTI_TEST: bool = true;

fn cleanup(A: &Vec<u32>, remaining: &mut HashMap<u32, u32>) -> Option<Vec<(u32, u32)>> {
    let mut x = A[0];
    let mut ops = Vec::new();
    for i in 1..A.len() {
        let a = A[i];
        let b = x - A[i];

        let nindices_for_val_a = remaining.entry(a).or_insert(0);

        if *nindices_for_val_a == 0 {
            continue; // Already taken. Skip it.
        }
        // OK, we have to take it.
        *nindices_for_val_a -= 1;

        let nindices_for_b = remaining.entry(b).or_insert(0);

        // The complement must exist...
        if *nindices_for_b == 0 {
            return None; // ...otherwise the initially chosen second value is bad.
        } else {
            ops.push((a, b));
            *nindices_for_b -= 1;
            x = a.max(b);
        }
    }

    return Some(ops);
}
fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let n = read.next::<usize>();
    let mut A: Vec<u32> = read.next_vec::<u32>(2 * n);
    A.sort_by_key(|a| Reverse(*a));

    let mut remaining = HashMap::<u32, u32>::with_capacity(2 * n - 1);

    // dbg!(&A, &remaining);

    for i in 1..2 * n {
        // Is there a method for it?
        for i in 1..2 * n {
            *remaining.entry(A[i]).or_insert(0) += 1;
        }
        // OK, we combine the maximum with this element now. We delete it from the remaining set.
        // eprintln!(
        //     "i = {} A[i] = {} rm[A[i]] = {}",
        //     i,
        //     A[i],
        //     remaining.get(&A[i]).unwrap()
        // );

        *remaining.entry(A[i]).or_insert(0) -= 1;

        if let Some(mut ops) = cleanup(&A, &mut remaining) {
            // Remember that the first operation was the selection of the maximum and the second element.
            ops.insert(0, (A[0], A[i]));
            println!("YES");
            println!("{}", A[0] + A[i]);
            for op in ops {
                println!("{} {}", op.0, op.1);
            }
            return;
        }

        remaining.clear();
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
