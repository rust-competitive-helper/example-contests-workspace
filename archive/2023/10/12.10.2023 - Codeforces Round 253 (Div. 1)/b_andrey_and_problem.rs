//{"name":"B. Andrey and Problem","group":"Codeforces - Codeforces Round 253 (Div. 1)","url":"https://codeforces.com/problemset/problem/442/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n0.1 0.2 0.3 0.8\n","output":"0.800000000000\n"},{"input":"2\n0.1 0.2\n","output":"0.260000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BAndreyAndProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut p = input.read_float_vec(n);
    let mut best = 0.0;
    p.sort_by(|a, b| b.partial_cmp(a).unwrap());

    p.iter().for_each(|x| best = f64::max(best, *x));
    if best >= 0.5 {
        out_line!(best);
    } else {
        let mut answer = 0.0;
        for to_pick in 1..=n {
            let mut all = 0.0;
            for i in 0..to_pick {
                let mut this_answer = 1.0;
                for j in 0..to_pick {
                    if i == j {
                        this_answer *= p[j];
                    } else {
                        this_answer *= (1.0 - p[j]);
                    }
                }
                all += this_answer;
            }
            if all < answer {
                break;
            } else {
                answer = all;
            }
        }
        out_line!(answer);
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
