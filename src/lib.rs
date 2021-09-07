use std::collections::HashMap;

#[allow(non_snake_case)]
fn longest_common_substring_matrix(A: &Vec<char>, B: &Vec<char>) -> Vec<Vec<u32>> {
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

fn longest_common_substring(A: &Vec<char>, B: &Vec<char>) -> u32 {
    longest_common_substring_matrix(A, B)[A.len()][B.len()]
}

// Output: (out-degrees, in-degrees)
fn get_degrees(g: &[Vec<usize>], also_in_degrees: bool) -> (Vec<usize>, Option<Vec<usize>>) {
    let n = g.len();
    let mut out_degrees = vec![0; n];

    let mut in_degrees = if also_in_degrees {
        vec![0; n]
    } else {
        vec![]
    };


    for u in 0..n {
        out_degrees[u] = g[u].len();

        if also_in_degrees {
            for v in &g[u] {
                debug_assert!(*v < n);
                in_degrees[*v] += 1;
            }
        }
    }

    if also_in_degrees {
        (out_degrees, Some(in_degrees))
    } else {
        (out_degrees, None)
    }
}

// `dag` should be an acyclic graph.
fn topological_sort(dag: &[Vec<usize>]) -> Vec<usize> {
    let n = dag.len();

    if n == 0 {
        return vec![];
    }

    if let (_, Some(in_degrees)) = get_degrees(&dag, true) {
        let sources: Vec<usize> = in_degrees
            .into_iter()
            .enumerate()
            .filter(|(_, in_degree)| *in_degree == 0)
            .map(|(v, _)| v).collect();

        let mut topo: Vec<i32> = vec![-1; n];

        fn dfs(dag: &[Vec<usize>], n: usize, topo: &mut Vec<i32>, pos: usize) {
            debug_assert_eq!(topo[n], -1);

            let mut my_pos = pos as i32;
            for neighbour in &dag[n] {
                if topo[*neighbour] == -1 {
                    dfs(dag, *neighbour, topo, pos)
                }

                my_pos = my_pos.max(topo[*neighbour] + 1);
            }
            topo[n] = my_pos;
        }

        for source in sources {
            if topo[source] == -1 {
                dfs(&dag, source, &mut topo, 0);
            }
        }

        topo.iter().map(|topo_pos| {
            debug_assert_ne!(*topo_pos, -1);
            *topo_pos as usize
        }).collect()
    } else {
        debug_assert!(false);
        vec![] // ease the compiler
    }
}

#[cfg(test)]
mod tests {
    use super::longest_common_substring;
    use crate::topological_sort;
    use std::io::{BufReader, Stdin, StdinLock, repeat, BufWriter, Write};
    use std::process::Stdio;

    #[test]
    fn test_lcs() {
        assert_eq!(longest_common_substring(&vec![], &vec![]), 0);
        assert_eq!(longest_common_substring(&vec![], &vec!['a']), 0);
        assert_eq!(longest_common_substring(&vec!['a'], &vec![]), 0);
        assert_eq!(longest_common_substring(&vec!['a'], &vec!['a']), 1);
        assert_eq!(longest_common_substring(&vec!['a', 'a'], &vec!['a']), 1);
        assert_eq!(longest_common_substring(&vec!['a', 'b'], &vec!['a']), 1);
        assert_eq!(longest_common_substring(&vec!['b', 'b'], &vec!['a']), 0);

        assert_eq!(
            longest_common_substring(&vec!['a', 'b', 'a'], &vec!['a', 'a']),
            2
        );

        assert_eq!(
            longest_common_substring(
                &vec!['a', 'b', 'b', 'b', 'a'],
                &vec!['a', 'a', 'b', 'b', 'a'],
            ),
            4
        );

        assert_eq!(
            longest_common_substring(
                &vec!['a', 'a', 'b', 'b', 'c', 'c', 'd', 'd', 'e', 'e'],
                &vec!['a', 'c', 'e'],
            ),
            3
        );
        assert_eq!(
            longest_common_substring(&vec!['a', 'b', 'a', 'b', 'c'], &vec!['x', 'b', 'y']),
            1
        );
    }

    type Graph = Vec<Vec<usize>>;

    #[allow(dead_code)]
    #[test]
    fn test_topo_interactive() {
        let mut r = Reader::new(BufReader::new(std::io::stdin()));
        let mut w = BufWriter::new(std::io::stderr());

       loop {
            let g = r.next_unweighted_digraph();


            writeln!(w, "G = {:?}", g).ok();
            writeln!(w, "Topo(G) = {:?}", topological_sort(&g)).ok();

           w.flush().ok();
        }
    }

    #[test]
    fn test_topo() {
        // If some asserts fail, it does not mean that the given topological sort is invalid.
        assert_eq!(topological_sort(&[vec![], vec![], vec![], vec![], vec![]]), vec![0; 5]);
        assert_eq!(topological_sort(&[vec![1], vec![2], vec![3], vec![4], vec![]]), vec![4, 3, 2, 1, 0]);
        assert_eq!(topological_sort(&[vec![1, 2], vec![3], vec![3], vec![4], vec![]]), vec![3, 2, 2, 1, 0]);
        assert_eq!(topological_sort(&[vec![1, 2], vec![3], vec![4], vec![4, 5], vec![5], vec![]]), vec![4, 3, 2, 2, 1, 0]);
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

        fn next_unweighted_digraph(&mut self) -> Graph {
            let n = self.next::<usize>();
            let m = self.next::<usize>();

            let mut g: Graph = vec![Vec::with_capacity(m); n];

            for _ in 0..m {
                let (u, v) = self.next_pair::<usize>();
                debug_assert!(u < n);
                debug_assert!(v < n);
                g[u].push(v);
            }

            g
        }
    }
}
