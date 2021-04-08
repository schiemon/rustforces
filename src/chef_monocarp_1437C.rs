#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

// -------------------------------------------------------------------------------------------------

const MULTI_TEST: bool = true;
const INF: u32 = std::primitive::u32::MAX >> 1;

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let n = read.next_token::<usize>();
    // This T has nothing to do with the T in the problem statement.
    // T[i + 1] := t_i
    let mut T = read.next_vec::<u32>(n);
    T.sort_unstable();

    // dp[d][t] - Minimal unpleasant value possible while taking out the first d + 1 dishes between
    // minute 0 and minute 2 * n - 1
    // dp[d][0] = INF because this is not possible. It is used as a sentinel.
    // dp[d][t] = min(dp[d][t], dp[d - 1][t - 1] + |T[d] - t|, dp[d][t - 1])

    // dp[n - 1][2 * n - 1] is the answer.
    let mut dp = vec![vec![INF; 2 * n]; n + 1];

    // The sentinel dish does not generate costs at all.
    for t in 0..2 * n {
        dp[0][t] = 0;
    }

    for d in 1..=n {
        for t in 1..2 * n {
            dp[d][t] = dp[d][t]
                .min(dp[d - 1][t - 1] + (T[d - 1] as i32 - t as i32).abs() as u32)
                .min(dp[d][t - 1]);
        }
    }

    println!("{}", dp[n][2 * n - 1]);
}

// -------------------------------------------------------------------------------------------------

//noinspection Duplicates, RsRedundantElse
pub fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let mut read = Reader::new(stdin.lock());
    let mut write = std::io::BufWriter::new(stdout.lock());

    let t = if MULTI_TEST { read.next_token::<u32>() } else { 1 };

    for _ in 0..t {
        solve(&mut read, &mut write);
    }
}
pub struct Reader<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}
//noinspection Duplicates
impl<B: std::io::BufRead> Reader<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }

    pub fn next_token<T: std::str::FromStr>(&mut self) -> T {
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
            v.push(self.next_token());
        }

        v
    }

    pub fn next_pair<T: std::str::FromStr>(&mut self) -> (T, T) {
        let first = self.next_token();
        let second = self.next_token();

        (first, second)
    }

    pub fn next_char_vec(&mut self) -> Vec<char> {
        let s = self.next_token::<String>();
        s.chars().collect()
    }
}
