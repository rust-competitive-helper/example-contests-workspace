//{"name":"A. Cows and Sequence","group":"Codeforces - Codeforces Round 174 (Div. 1)","url":"https://codeforces.com/problemset/problem/283/A","interactive":false,"timeLimit":1500,"tests":[{"input":"5\n2 1\n3\n2 3\n2 1\n3\n","output":"0.500000\n0.000000\n1.500000\n1.333333\n1.500000\n"},{"input":"6\n2 1\n1 2 20\n2 2\n1 2 -3\n3\n3\n","output":"0.500000\n20.500000\n14.333333\n12.333333\n17.500000\n17.000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACowsAndSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const MAX_LENGTH: usize = 2 * 100_005;

fn solve(input: &mut Input, _test_case: usize) {
    let operations = input.read_size();
    let mut current_sum = 0_i64;
    let mut elements = vec![0];
    let mut sum_at = vec![0; MAX_LENGTH];
    for _ in 0..operations {
        let _type = input.read_size();
        match _type {
            1 => {
                let length = input.read_size();
                let value = input.read_long();
                current_sum += (value * length as i64);
                sum_at[length - 1] += value;
            }
            2 => {
                let value = input.read_long();
                elements.push(value);
                current_sum += value;
            }
            3 => {
                let popped = elements.pop().unwrap();
                current_sum -= popped;
                current_sum -= sum_at[elements.len()];
                if elements.len() > 0 {
                    sum_at[elements.len() - 1] += sum_at[elements.len()];
                }
                sum_at[elements.len()] = 0;
            }
            _ => unreachable!("invalid input"),
        }
        let current_size = elements.len();
        let output = format!("{:.10}", current_sum as f64 / current_size as f64);
        out_line!(output);
    }
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
