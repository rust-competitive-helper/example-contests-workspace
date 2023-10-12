//{"name":"A. Writing Code","group":"Codeforces - Codeforces Round 302 (Div. 1)","url":"https://codeforces.com/problemset/problem/543/A","interactive":false,"timeLimit":3000,"tests":[{"input":"3 3 3 100\n1 1 1\n","output":"10\n"},{"input":"3 6 5 1000000007\n1 2 3\n","output":"0\n"},{"input":"3 5 6 11\n1 2 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AWritingCode"}}}

use std::collections::{BTreeMap, HashMap};

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    lines_written: usize,
    bugs_produced: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m, b) = (input.read_size(), input.read_size(), input.read_size());
    let MOD = input.read_size();
    let a: Vec<usize> = input.read_vec(n);

    let mut dp = [[0; 1505]; 1505];
    dp[0][0] = 1;
    for index in 0..n {
        for lines_written in 0..=m {
            for bugs_produced in 0..=b {
                let previous_dp = dp[lines_written][bugs_produced];
                let new_bugs = a[index];
                let mut new_dp = dp[lines_written + 1]
                    .get_mut(bugs_produced + new_bugs)
                    .unwrap();
                *new_dp += previous_dp;
                if *new_dp >= MOD {
                    *new_dp -= MOD;
                }
            }
        }
    }

    let mut answer = 0;
    for b in 0..=b {
        answer += dp[m][b];
        if answer >= MOD {
            answer -= MOD;
        }
    }
    out_line!(answer);
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
