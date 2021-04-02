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
    let m = read.next::<usize>();

    let A: Vec<u32> = read.next_vec(n);
    let B: Vec<u32> = read.next_vec(n);
    let C: Vec<u32> = read.next_vec(m);

    let mut mp_color_nplanks = HashMap::<u32, HashSet<usize>>::new();
    let mut colors = HashMap::<u32, usize>::new();
    let mut nuncolored = 0;
    let mut unemployed_painters = Vec::new();

    for i in 0..n {
        colors.insert(B[i], i);
        if A[i] != B[i] {
            nuncolored += 1;
            mp_color_nplanks
                .entry(B[i])
                .or_insert(HashSet::new())
                .insert(i);
        }
    }

    // dbg!(&mp_color_nplanks);

    let mut painter_to_fence = vec![0; m];

    fn employ(
        painter_to_fence: &mut Vec<usize>,
        unemployed_painters: &mut Vec<usize>,
        to_fence: usize,
    ) {
        for unemployed_painter in unemployed_painters.iter() {
            painter_to_fence[*unemployed_painter] = to_fence;
        }

        unemployed_painters.clear();
    }

    for i in 0..m {
        if mp_color_nplanks.contains_key(&C[i]) {
            let fences_to_be_colored = mp_color_nplanks.get_mut(&C[i]).unwrap();

            if fences_to_be_colored.len() > 0 {
                // There are fences that you can paint. Take an abitrary one.
                let fence_to_be_colored_ref = *fences_to_be_colored.iter().next().unwrap();
                let fence_to_be_colored =
                    fences_to_be_colored.take(&fence_to_be_colored_ref).unwrap();
                painter_to_fence[i] = fence_to_be_colored + 1;
                nuncolored -= 1;
                employ(
                    &mut painter_to_fence,
                    &mut unemployed_painters,
                    fence_to_be_colored + 1,
                );
                continue;
            }
        }

        if colors.contains_key(&C[i]) {
            // There is indeed an already painted fence. Send the unemployed painter to that fence.
            let repr = *colors.get(&C[i]).unwrap();
            painter_to_fence[i] = repr + 1;
            employ(&mut painter_to_fence, &mut unemployed_painters, repr + 1);
        } else {
            unemployed_painters.push(i);
        }
    }

    // dbg!(&painter_to_fence, &nuncolored);

    if unemployed_painters.len() == 0 && nuncolored == 0 {
        println!("YES");
        for p in painter_to_fence {
            print!("{} ", p);
        }
        println!();
    } else {
        println!("NO");
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
