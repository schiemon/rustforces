#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cmp::{max, min};
use std::fmt::Display;
use std::io::{BufRead, BufWriter, stdin, stdout, Write};

const MULTI_TEST: bool = true;

fn output_vec<T: Display, W: Write>(write: &mut BufWriter<W>, v: Vec<T>) {
    write!(write, "{}\n", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn solve<B: BufRead, W: Write>(
    read: &mut Reader<B>,
    write: &mut BufWriter<W>,
) {
    // Number of rows and columns.
    let (n, m) = read.next_pair::<usize>();

    /*
        Explanation:

        Rahul wants to get a seat S for which the distance to the farthest not-painted seat T is minimized.
        For every seat the corners are the only seat that matter since only one of those will maximize the distance.
        This because every other not-corner candidate T' (for maximum distance) you can translate T' away and increase the distance.
        This translation is dependent on how S is positioned relative to T':

               |  T'
               | /
        ------ T' ------
               |
        S      |
     */

    let mut dist_points: Vec<usize> = Vec::with_capacity(n * m);
    for r in 0..n {
        for c in 0..m {
            let dist = max(r, n - 1 - r) + max(c, m - 1 - c);
            dist_points.push(dist);
        }
    }

    dist_points.sort();

    output_vec(write, dist_points);
}

// Accepted - 15 ms	- 5084 KB
pub fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let mut read = Reader::new(stdin.lock());
    let mut write = BufWriter::new(stdout.lock());

    let t = if MULTI_TEST { read.next::<u32>() } else { 1 };

    for _ in 0..t {
        solve(&mut read, &mut write);
    }

    write.flush().unwrap();
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

    pub fn next_char_vec(&mut self, n: usize) -> Vec<char> {
        let s = self.next::<String>();
        let cv = s.chars().collect::<Vec<char>>();
        assert_eq!(cv.len(), n);
        cv
    }

    pub fn next_pair<T: std::str::FromStr>(&mut self) -> (T, T) {
        let first = self.next();
        let second = self.next();

        (first, second)
    }
}

fn print_matrix<T: std::fmt::Display>(A: Vec<Vec<T>>) {
    let n = A.len();
    if n == 0 {
        println!("[]");
    } else {
        let m = A[0].len();

        let mut cell_width = 1;

        for i in 0..n {
            for j in 0..m {
                cell_width = cell_width.max(format!("{}", A[i][j]).len());
            }
        }

        cell_width += 2;

        for i in 0..n {
            for j in 0..m {
                print!("{:>width$}", A[i][j], width = cell_width);
            }
            println!();
        }
    }
}