#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// -------------------------------------------------------------------------------------------------

const MULTI_TEST: bool = true;

fn solve<B: std::io::BufRead, W: std::io::Write>(
    read: &mut Reader<B>,
    write: &mut std::io::BufWriter<W>,
) {
    let n = read.next::<usize>();

    let A = read.next_char_vec();
    let B = read.next_char_vec();

    assert_eq!(A.len(), n);
    assert_eq!(B.len(), n);

    let mut B_inversed = Vec::with_capacity(n);

    for i in 0..n {
        B_inversed.push(
            if B[i] == '0' {
                '1'
            } else {
                '0'
            }
        );
    }

    let mut balance = 0;
    let mut curr = 0;


    // Check that for every two consecutive valid prefixes the ability to reach the target string.
    // (Either we do nothing or we inverse that segment).
    let mut last: i32 = -1;
    let mut ok = true;
    while curr < n {
        balance += if A[curr] == '0' {
            -1
        } else {
            1
        };

        if balance == 0 {
            let l = (last + 1) as usize;
            let r = curr + 1;
            if &A[l..r] != &B[l..r] && &A[l..r] != &B_inversed[l..r] {
                ok = false;
                break;
            }

            last = curr as i32;
        }
        curr += 1;
    }

    // Check rest.
    let rest = (last + 1) as usize;
    if rest < n && &A[rest..] != &B[rest..] {
        ok = false;
    }

    println!("{}", if ok {
        "YES"
    } else {
        "NO"
    });
}

// -------------------------------------------------------------------------------------------------

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

    pub fn next_char_vec(&mut self) -> Vec<char> {
        let s = self.next::<String>();
        s.chars().collect()
    }
}
