#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

const MULTI_TEST: bool = true;

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let n = read.next::<usize>();
    let mut C = Vec::with_capacity(n);

    for _ in 0..n {
        let (t, x) = read.next_pair::<u32, i32>();
        C.push((t, x));
    }

    let mut visited = 0;

    let mut curr_target = 0;
    let mut direction = 0i32;
    let mut last_pos = 0i32;
    let mut last_time = 0;

    for i in 0..n {
        let (my_time, my_target) = C[i];
        let t_delta_prev = my_time - last_time;
        let mut my_pos = last_pos + direction * t_delta_prev as i32;

        if direction >= 0 {
            my_pos = my_pos.min(curr_target);
        } else {
            my_pos = my_pos.max(curr_target);
        }

        /*
        eprintln!(
            "my_pos = {} my_time = {} my_target = {} dir = {} vis= {}",
            my_pos, my_time, my_target, direction, visited
        );
         */

        if my_pos == curr_target {
            curr_target = my_target;
            direction = if my_target - my_pos > 0 {
                1
            } else if my_target - my_pos < 0 {
                -1
            } else {
                0
            }
        }

        // dbg!(curr_target, direction);

        let mut next_pos = curr_target;

        if i < n - 1 {
            let next_time = C[i + 1].0 as i32;
            let pos_when_uninterrupted = my_pos + direction * (next_time - my_time as i32);
            if direction >= 0 {
                next_pos = next_pos.min(pos_when_uninterrupted); } else {
                next_pos = next_pos.max(pos_when_uninterrupted);
            }
        }

        let mn_pos_visited = my_pos.min(next_pos);
        let mx_pos_visited = my_pos.max(next_pos);

        /*
        eprintln!(
            "({}, {}) will be visited until the next command.",
            mn_pos_visited, mx_pos_visited
        );
        eprintln!();*/

        if (mn_pos_visited..=mx_pos_visited).contains(&my_target) {
            visited += 1;
        }

        last_pos = my_pos;
        last_time = my_time;
    }

    println!("{}", visited);
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

    fn next<T: std::str::FromStr>(&mut self) -> T {
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

    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.next());
        }

        v
    }

    fn next_pair<U: std::str::FromStr, T: std::str::FromStr>(&mut self) -> (U, T) {
        let first = self.next();
        let second = self.next();

        (first, second)
    }
}
