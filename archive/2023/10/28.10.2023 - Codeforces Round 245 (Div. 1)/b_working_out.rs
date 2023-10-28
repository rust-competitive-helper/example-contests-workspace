//{"name":"B. Working out","group":"Codeforces - Codeforces Round 245 (Div. 1)","url":"https://codeforces.com/problemset/problem/429/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n100 100 100\n100 1 100\n100 100 100\n","output":"800\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BWorkingOut"}}}

use algo_lib::collections::array2d::Array2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let grid: Array2d<i64> = Array2d::read(input);
    let mut from_top_left = grid.clone();
    let mut from_bottom_left = grid.clone();
    let mut from_top_right = grid.clone();
    let mut from_bottom_right = grid.clone();
    let (n, m) = (grid.d1(), grid.d2());
    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0 {
                continue;
            }
            let from_above = if i == 0 {
                None
            } else {
                Some(from_top_left[i - 1][j])
            };
            let from_left = if j == 0 {
                None
            } else {
                Some(from_top_left[i][j - 1])
            };

            from_top_left[i][j] += std::cmp::max(from_above, from_left).unwrap();
        }
    }

    for i in 0..n {
        for j in (0..m).rev() {
            if i == 0 && j == m - 1 {
                continue;
            }
            let from_above = if i == 0 {
                None
            } else {
                Some(from_top_right[i - 1][j])
            };
            let from_right = if j == m - 1 {
                None
            } else {
                Some(from_top_right[i][j + 1])
            };

            from_top_right[i][j] += std::cmp::max(from_above, from_right).unwrap();
        }
    }

    for i in (0..n).rev() {
        for j in 0..m {
            if i == n - 1 && j == 0 {
                continue;
            }
            let from_below = if i == n - 1 {
                None
            } else {
                Some(from_bottom_left[i + 1][j])
            };
            let from_left = if j == 0 {
                None
            } else {
                Some(from_bottom_left[i][j - 1])
            };

            from_bottom_left[i][j] += std::cmp::max(from_below, from_left).unwrap();
        }
    }

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if i == n - 1 && j == m - 1 {
                continue;
            }
            let from_below = if i == n - 1 {
                None
            } else {
                Some(from_bottom_right[i + 1][j])
            };
            let from_right = if j == m - 1 {
                None
            } else {
                Some(from_bottom_right[i][j + 1])
            };

            from_bottom_right[i][j] += std::cmp::max(from_below, from_right).unwrap();
        }
    }

    let mut answer = 0;
    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            let first_from_top = if i == 0 { 0 } else { from_top_left[i - 1][j] };
            let second_from_below = if i == n - 1 {
                0
            } else {
                from_bottom_left[i + 1][j]
            };
            let first_from_left = if j == 0 { 0 } else { from_top_left[i][j - 1] };
            let second_from_left = if j == 0 {
                0
            } else {
                from_bottom_left[i][j - 1]
            };
            let first_go_below = if i == n - 1 {
                0
            } else {
                from_bottom_right[i + 1][j]
            };
            let first_go_right = if j == m - 1 {
                0
            } else {
                from_bottom_right[i][j + 1]
            };
            let second_go_above = if i == 0 { 0 } else { from_top_right[i - 1][j] };
            let second_go_right = if j == m - 1 {
                0
            } else {
                from_top_right[i][j + 1]
            };
            answer.maxim(first_from_top + first_go_below + second_from_left + second_go_right);
            answer.maxim(first_from_left + first_go_right + second_from_below + second_go_above);
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
