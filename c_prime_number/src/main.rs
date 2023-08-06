//{"name":"C. Prime Number","group":"Codeforces - Codeforces Round 209 (Div. 2)","url":"https://codeforces.com/problemset/problem/359/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2\n2 2\n","output":"8\n"},{"input":"3 3\n1 2 3\n","output":"27\n"},{"input":"2 2\n29 29\n","output":"73741817\n"},{"input":"4 5\n0 0 0 0\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPrimeNumber"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::min;
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn solve(input: &mut Input, _test_case: usize) {
    let (n, x) = (input.read_size(), input.read_long());
    let exponents: Vec<i64> = input.read_vec(n);
    let sum: i64 = exponents.iter().sum();
    let mut count: HashMap<i64, i64> = HashMap::new();

    let values_to_consider = {
        let mut res = Vec::with_capacity(n);
        let sum: i64 = exponents.iter().sum();
        for e in exponents {
            res.push(sum - e);
        }
        res
    };

    for e in values_to_consider {
        let mut current_value = e;
        let mut current_count = count.entry(current_value).or_default();
        *current_count += 1;
        assert!(*current_count <= x);

        // Potentially propagate.
        while *current_count == x {
            *current_count = 0;

            let next_value = current_value + 1;
            let next_count = count.entry(next_value).or_default();
            *next_count += 1;

            current_value = next_value;
            current_count = next_count;
        }
    }

    let binary_exponentiation = |mut b: i64, mut e: i64, m: i64| {
        let mut res = 1;
        b %= m;
        while e != 0 {
            if e & 1 == 1 {
                res = (res * b) % m;
            }
            b = (b * b) % m;
            e /= 2;
        }
        res
    };

    let smallest = count
        .into_iter()
        .filter(|&(e, c)| c != 0)
        .map(|(e, c)| e)
        .min()
        .unwrap();
    let answer = min(smallest, sum);
    out_line!(binary_exponentiation(x, answer, MOD));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
