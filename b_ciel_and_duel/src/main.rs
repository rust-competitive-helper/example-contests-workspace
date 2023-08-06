//{"name":"B. Ciel and Duel","group":"Codeforces - Codeforces Round 190 (Div. 1)","url":"https://codeforces.com/problemset/problem/321/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3\nATK 2000\nDEF 1700\n2500\n2500\n2500\n","output":"3000\n"},{"input":"3 4\nATK 10\nATK 100\nATK 1000\n1\n11\n101\n1001\n","output":"992\n"},{"input":"2 4\nDEF 0\nATK 0\n0\n0\n1\n1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCielAndDuel"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    
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
