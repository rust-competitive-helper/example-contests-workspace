//{"name":"A. Арбуз","group":"Codeforces - Codeforces Beta Round #4 (Дивизион 2)","url":"https://codeforces.com/problemset/problem/4/A","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AArbuz"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let res = if n % 2 == 0 && n > 2 { "YES" } else { "NO" };
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
