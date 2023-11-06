//{"name":"A. Vasya and Robot","group":"Codeforces - Codeforces Round 206 (Div. 1)","url":"https://codeforces.com/problemset/problem/354/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4 4 19 1\n42 3 99\n","output":"576\n"},{"input":"4 7 2 3 9\n1 2 3 4\n","output":"34\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AVasyaAndRobot"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let (cost_l, cost_r) = (input.read_long(), input.read_long());
    let (extra_l, extra_r) = (input.read_long(), input.read_long());
    let weights = input.read_long_vec(n);

    let prefix_sum = {
        let mut res = vec![0];
        for &w in &weights {
            res.push(w + res.last().unwrap_or(&0));
        }
        res
    };

    let suffix_sum = {
        let mut res = vec![0];
        for &w in weights.iter().rev() {
            res.push(w + res.last().unwrap_or(&0));
        }
        res
    };

    let mut best_answer = i64::MAX;
    for taken_from_left in 0..=n {
        let taken_from_right = n - taken_from_left;
        let cost_from_left = cost_l * prefix_sum[taken_from_left];
        let cost_from_right = cost_r * suffix_sum[taken_from_right];

        let extra = {
            if taken_from_left > taken_from_right {
                (taken_from_left - (taken_from_right + 1)) as i64 * extra_l
            } else if taken_from_left == taken_from_right {
                0
            } else {
                (taken_from_right - (taken_from_left + 1)) as i64 * extra_r
            }
        };

        best_answer.minim(cost_from_left + cost_from_right + extra);
    }

    out_line!(best_answer);
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
