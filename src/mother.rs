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
    let mut A: Vec<u32> = Vec::with_capacity(n);

    for i in 0..n {
        A.push(read.next::<u32>());
    }
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
                /*
                    Why do we have to use this ugly function here?
                    Well, we want to buffer 'static strings in a SplitWhitespace member.
                    We want them to be 'static because we want their lifetimes to be independent
                    on the lifetime of the mutable self-reference provided in this method.
                    This i nturn requires the lifetime of the self-reference to be at
                    least as endurable as the reference to the SplitWhitespace object.
                    This would be inpractical because we would not be able to use the mutable borrow twice as Rust
                    would restrict us to at most one mutable 'static reference.
                */

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
