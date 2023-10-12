//{"name":"A. DZY Loves Sequences","group":"Codeforces - Codeforces Round #FF (Div. 1)","url":"https://codeforces.com/problemset/problem/446/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n7 2 3 1 5 6\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADZYLovesSequences"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let values = input.read_size_vec(n);
    let mut best_ending_at = vec![1; n];
    let mut best_starting_at = vec![1; n];
    for i in 1..n {
        if values[i] > values[i - 1] {
            best_ending_at[i] = best_ending_at[i - 1] + 1;
        }
    }
    for i in (0..(n - 1)).rev() {
        if values[i] < values[i + 1] {
            best_starting_at[i] = best_starting_at[i + 1] + 1;
        }
    }

    let mut answer = 0;
    for i in 0..n {
        let left_index = i.wrapping_sub(1);
        let right_index = i.wrapping_add(1);
        let from_left = *best_ending_at.get(left_index).unwrap_or(&0);
        let from_right = *best_starting_at.get(right_index).unwrap_or(&0);
        if *values.get(left_index).unwrap_or(&0)
            < values.get(right_index).unwrap_or(&usize::MAX) - 1
        {
            answer.maxim(from_left + 1 + from_right);
        } else {
            answer.maxim(std::cmp::max(from_left, from_right) + 1);
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
