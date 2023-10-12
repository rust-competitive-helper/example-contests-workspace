//{"name":"A. Two Vessels","group":"Codeforces - Codeforces Round 895 (Div. 3)","url":"https://codeforces.com/contest/1872/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 7 2\n17 4 3\n17 17 1\n17 21 100\n1 100 1\n97 4 3\n","output":"1\n3\n0\n1\n50\n16\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATwoVessels"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (a, b, c) = (input.read_int(), input.read_int(), input.read_int());
    let delta = (a.max(b) - a.min(b)) as f64 / 2.0;
    out_line!((delta / c as f64).ceil());
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
