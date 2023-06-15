//{"name":"C. Ranom Numbers","group":"Codeforces - Educational Codeforces Round 150 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1841/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nDAAABDCA\nAB\nABCDEEDCBA\nDDDDAAADDABECD\n","output":"11088\n10010\n31000\n15886\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRanomNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::max;

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read_string();
    let n = s.len();

    let score = vec![1, 10, 100, 1000, 10000];

    let mut dp = vec![vec![vec![i64::MIN; 2]; 5]; n + 1];
    match s.bytes().next_back() {
        None => {}
        Some(c) => {
            let m = match c {
                b'A' => 0,
                b'B' => 1,
                b'C' => 2,
                b'D' => 3,
                b'E' => 4,
                _ => unreachable!(),
            };
            dp[n - 1][m][0] = score[m];
            dp[n - 1][0][1] = score[0];
            dp[n - 1][1][1] = score[1];
            dp[n - 1][2][1] = score[2];
            dp[n - 1][3][1] = score[3];
            dp[n - 1][4][1] = score[4];
        }
    }

    for (i, c) in s.bytes().enumerate().rev().skip(1) {
        let c = match c {
            b'A' => 0,
            b'B' => 1,
            b'C' => 2,
            b'D' => 3,
            b'E' => 4,
            _ => unreachable!(),
        };
        for new_char in 0..5 {
            for max_before in 0..5 {
                if new_char >= max_before {
                    let new_max = new_char;
                    if new_char == c {
                        // Do nothing.
                        dp[i][new_max][0] = max(
                            dp[i][new_max][0],
                            dp[i + 1][max_before][0] + score[new_char],
                        );
                        dp[i][new_max][1] = max(
                            dp[i][new_max][1],
                            dp[i + 1][max_before][1] + score[new_char],
                        );
                    } else {
                        dp[i][new_max][1] = max(
                            dp[i][new_max][1],
                            dp[i + 1][max_before][0] + score[new_char],
                        );
                    }
                } else {
                    let new_max = max_before;
                    if new_char == c {
                        dp[i][new_max][0] = max(
                            dp[i][new_max][0],
                            dp[i + 1][max_before][0].saturating_sub(score[new_char]),
                        );
                        dp[i][new_max][1] = max(
                            dp[i][new_max][1],
                            dp[i + 1][max_before][1].saturating_sub(score[new_char]),
                        );
                    } else {
                        dp[i][new_max][1] = max(
                            dp[i][new_max][1],
                            dp[i + 1][max_before][0].saturating_sub(score[new_char]),
                        );
                    }
                }
            }
        }
    }

    let answer = {
        let mut answer = i64::MIN;
        for m in 0..5 {
            answer = max(answer, dp[0][m][0]);
            answer = max(answer, dp[0][m][1]);
        }
        answer
    };
    out_line!(answer);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
