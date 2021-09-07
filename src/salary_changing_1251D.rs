use std::{io::{BufRead, BufReader, Stdin}, u32};

fn read_vec<F: std::str::FromStr>(r: &mut BufReader<Stdin>, n: usize) -> Vec<F>
where
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut nums_str = String::new();
    r.read_line(&mut nums_str);
    let nums_vec: Vec<F> = nums_str
        .split_whitespace()
        .map(|x| x.parse::<F>().unwrap())
        .collect();

    assert_eq!(nums_vec.len(), n);

    nums_vec
}

fn read<F>(r: &mut BufReader<Stdin>) -> F
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_vec::<F>(r, 1)[0]
}

fn read_pair<F>(r: &mut BufReader<Stdin>) -> (F, F)
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    match read_vec::<F>(r, 2).as_slice() {
        [a, b, ..] => (*a, *b), // We can only get references. Copy this.
        [..] => unreachable!("Called read(u32) did not asserted the length of the input."),
    }
}

fn read_quadruple<F>(r: &mut BufReader<Stdin>) -> (F, F, F, F)
where
    F: std::str::FromStr + Copy,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    match read_vec::<F>(r, 4).as_slice() {
        [a, b, c, d, ..] => (*a, *b, *c, *d), // We can only get references. Copy this.
        [..] => unreachable!("Called read(u32) did not asserted the length of the input."),
    }
}

pub fn solve(r: &mut BufReader<Stdin>) {
    let (n, s) = read_pair::<usize>(r);

    assert!(n % 2 == 1);

    let s = s as u32;
    let mut salaries = vec![(0, 0); n];
    let mut lo = u32::MAX;
    let mut hi = u32::MIN;

    for i in 0..n {
        salaries[i] = read_pair::<u32>(r);
        lo = lo.min(salaries[i].0);
        hi = hi.max(salaries[i].1);
    }

    salaries.sort_unstable();

    let rest = (s as i32 - salaries.iter().map(|x| x.0).sum::<u32>() as i32) as u32;
    // println!("New testcase; salaries= {:?}, rest = {}", salaries, rest);

    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        let mut doable = false; // Lets be pessimistic :).
        // Can we have a median with a salary at least mid?

        // Let L be the half of elements with a minimum salary strictly less than mid.
        // Let R be the half of elements with a minimum salary greater or equal to mid.
        // Let r be the index of the first element in R.

        // If R > L, then we know this is not possible because we cannot decrease the size of R.
        // If R < L, then we iterate through L to "raise" L - R elements (T) from L to R.
        //     If we cannot do this, this is impossible.
        //     (We can do this if we can cover the additional cost sum(salaries[n / 2] - t.1 for t in T) with s - sum(x.1 for x in salaries))
        // If R == L, then we are happy.
        
        // First, we have to find r in [0, n - 1].
        let mut lolo = 0;
        let mut hihi = n;

        while lolo < hihi {
            let midmid = lolo + (hihi - lolo) / 2;
            
            if salaries[midmid].0 >= mid {
                hihi = midmid as usize;
            } else {
                lolo = midmid + 1 as usize;
            }
        }

        let r = lolo;
        // println!("salaries = {:?} r = {} mid = {}",salaries, r, mid);
        if r == n / 2{
            // R = L.
            doable = true;
        } else if r > n / 2{
            // R < L.
            let mut transferred = 0;
            let need_transferred = r - n / 2;
            let mut transfer_cost = 0;
            let mut i = r - 1;

            while transferred < need_transferred {
                // If the salary can be raised, raise it.
                if salaries[i].1 >= mid {
                    transferred += 1;
                    // The raising cost is the difference to the targeted salary.
                    transfer_cost += mid - salaries[i].0;
                }

                if i == 0 {
                    break;
                }

                i -= 1;
            }

            // println!("transferred = {}; transfer_cost = {}; need_transferred = {}", transferred, transfer_cost, need_transferred);
            if transferred == need_transferred && transfer_cost <= rest {
                doable = true;
            }
        } else {
            // R > L.
        }

        // println!("doable = {}", doable);

        if doable {
            // answer >= mid
            lo = mid;
        } else {
            // answer < mid
            hi = mid - 1;
        }
    }

    println!("{}", lo);
}

pub fn main() {
    const MULTI_TEST: bool = true;
    let r = &mut BufReader::new(std::io::stdin());

    let t = if MULTI_TEST { read::<u32>(r) } else { 1 };

    for _ in 0..t {
        solve(r);
    }
}
