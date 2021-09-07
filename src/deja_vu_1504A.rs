#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

const MULTI_TEST: bool = true;

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let mut S = read.next_char_vec();
    let n = S.len();

    let mut ok = false;
    let mut idx_to_break = 0;
    for i in 0..n {
        if S[i] != 'a' {
            ok = true;
            idx_to_break = i;
        }
    }

    if ok {
        println!("YES");
        let mut extra: i32 = 0;
        if n % 2 == 0 {
            if idx_to_break == n / 2 - 1 {
                extra = 1;
            }
        }
        S.insert((n as i32 - 1 - idx_to_break as i32 + extra) as usize, 'a');
        for ch in S {
            print!("{}", ch);
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

    pub fn next_char_vec(&mut self) -> Vec<char> {
        let s = self.next::<String>();
        let cv = s.chars().collect::<Vec<char>>();
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
