#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

const MULTI_TEST: bool = false;

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

fn lcs_dp(A: Vec<char>, B: Vec<char>) -> Vec<Vec<u32>> {
    let n = A.len();
    let m = B.len();

    let mut dp = (0..n + 1)
        .map(|_| vec![0; m + 1])
        .collect::<Vec<Vec<u32>>>();

    for i in 1..n + 1 {
        for j in 1..m + 1 {
            dp[i][j] = if A[i - 1] == B[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            }
        }
    }

    dp
}

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let (n, m) = read.next_pair::<usize>();
    let A = read.next_char_vec(n);
    let B = read.next_char_vec(m);

    let mut S = (0..n + 1)
        .map(|_| vec![0; m + 1])
        .collect::<Vec<Vec<i32>>>();

    for i in 1..n + 1 {
        S[i][0] = -(i as i32);
    }

    for j in 1..m + 1 {
        S[0][j] = -(j as i32);
    }

    let mut S_max = 0;

    for i in 1..n + 1 {
        for j in 1..m + 1 {
            S[i][j] = -1 + (S[i - 1][j]).max(S[i][j - 1]);

            if A[i - 1] == B[j - 1] {
                S[i][j] = S[i][j].max(S[i - 1][j - 1].max(0) + 2);
            }

            S_max = S_max.max(S[i][j]);
        }
    }
    /*
    println!(
        "{:?}",
        lcs_dp(
            String::from("GAC").chars().collect(),
            String::from("AGCAT").chars().collect()
        )
    );
    */

    // print_matrix(S);

    println!("{}", S_max);
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
