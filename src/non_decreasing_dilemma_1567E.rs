#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cmp::{max, min};
use std::io::Write;

const MULTI_TEST: bool = false;

#[derive(Clone)]
struct NodeInfo {
    sl: u64,
    pref_sl: u64,
    suf_sl: u64,
}

impl Default for NodeInfo {
    fn default() -> Self {
        NodeInfo {
            sl: 0,
            pref_sl: 0,
            suf_sl: 0,
        }
    }
}

struct SegmentTree {
    a: Vec<u32>,
    n: usize,
    t: Vec<NodeInfo>,
}

impl SegmentTree {
    fn new(a: Vec<u32>) -> SegmentTree {
        let n = a.len();
        let mut st = SegmentTree {
            n: a.len(),
            a,
            t: (0..4 * n).map(|_| NodeInfo::default()).collect(),
        };

        st.__build(1, 0, n - 1);

        st
    }

    // Overthink this method to implement private methods.
    fn __build(&mut self, tv: usize, l: usize, r: usize) {
        if l == r {
            self.t[tv] = NodeInfo {
                sl: 1,
                pref_sl: 1,
                suf_sl: 1,
            }
        } else {
            let m = l + (r - l) / 2;
            self.__build(2 * tv, l, m);
            self.__build(2 * tv + 1, m + 1, r);

            self.t[tv] = self.__combine_ni(&self.t[2 * tv], &self.t[2 * tv + 1], l, m, r)
        }
    }

    fn __combine_ni(&self, l_ni: &NodeInfo, r_ni: &NodeInfo, l: usize, m: usize, r: usize) -> NodeInfo {
        let halves_connected = self.a[m] <= self.a[m + 1];

        let mut ni = NodeInfo::default();

        // sl
        ni.sl = l_ni.sl + r_ni.sl + l_ni.suf_sl * r_ni.pref_sl * (halves_connected as u64);

        // pref_sl
        if (l_ni.pref_sl == (m - l + 1) as u64) && halves_connected {
            ni.pref_sl = l_ni.pref_sl + r_ni.pref_sl
        } else {
            ni.pref_sl = l_ni.pref_sl
        }

        // suf_sl
        if (r_ni.suf_sl == (r - (m + 1) + 1) as u64) && halves_connected {
            ni.suf_sl = r_ni.suf_sl + l_ni.suf_sl
        } else {
            ni.suf_sl = r_ni.suf_sl
        }

        ni
    }

    fn __sl(&mut self, tv: usize, l: usize, r: usize, ql: usize, qr: usize) -> NodeInfo {
        if ql > qr {
            return NodeInfo::default();
        }

        if ql == l && qr == r {
            return self.t[tv].clone();
        }

        let m = l + (r - l) / 2;

        let l_ni = self.__sl(2 * tv, l, m, ql, min(m, qr));
        let r_ni = self.__sl(2 * tv + 1, m + 1, r, max(m + 1, ql), qr);

        self.__combine_ni(&l_ni, &r_ni, l, m, r)
    }

    fn sl(&mut self, l: usize, r: usize) -> u64 {
        self.__sl(1, 0, self.n - 1, l, r).sl
    }

    fn __update(&mut self, tv: usize, l: usize, r: usize, ql: usize, qr: usize, val: u32) {
        if ql > qr { return; }

        debug_assert!(/* 0 <= l && */l <= r && r < self.n);

        if l == r {
            self.a[l] = val;
        } else {
            let m = l + (r - l) / 2;

            self.__update(2 * tv, l, m, ql, min(m, qr), val);
            self.__update(2 * tv + 1, m +    1, r, max(m + 1, ql), qr, val);
            self.t[tv] = self.__combine_ni(&self.t[2 * tv], &self.t[2 * tv + 1], l, m, r);
        }
    }

    fn update(&mut self, ql: usize, qr: usize, v: u32) {
        self.__update(1, 0, self.n - 1, ql, qr, v)
    }
}


fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let (n, q) = read.next_pair::<usize>();
    let a = read.next_vec(n);

    let mut st = SegmentTree::new(a);

    for i in 0..q {
        let op: u8 = read.next();

        if op == 1 {
            let i: usize = read.next();
            let x = read.next();

            st.update(i - 1, i - 1, x);
        } else {
            let l: usize = read.next();
            let r: usize = read.next();
            write.write_fmt(format_args!("{}\n", st.sl(l - 1, r - 1))).unwrap();
        }
    }
}

// Accepted - 312 ms - 27112 KB
pub fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let mut read = Reader::new(stdin.lock());
    let mut write = std::io::BufWriter::new(stdout.lock());

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