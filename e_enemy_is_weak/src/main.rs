//{"name":"E. Enemy is weak","group":"Codeforces - Codeforces Beta Round 57 (Div. 2)","url":"https://codeforces.com/problemset/problem/61/E","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n3 2 1\n","output":"1\n"},{"input":"3\n2 3 1\n","output":"0\n"},{"input":"4\n10 8 3 1\n","output":"4\n"},{"input":"4\n1 5 4 3\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEnemyIsWeak"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::treap::{SizePayload, TreapNode};
use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let values: Vec<i32> = input.read_vec(n);

    let mut sorted = values.clone();
    sorted.sort_unstable();
    let get_id = |x: i32| sorted.binary_search(&x).unwrap();

    let mut fenwick_left = FenwickTree::new(n);
    fenwick_left.add(get_id(values[0]), 1);
    let mut fenwick_right = FenwickTree::new(n);
    for i in 1..n {
        fenwick_right.add(get_id(values[i]), 1);
    }

    let mut count: i64 = 0;
    for x in values.into_iter().take(n - 1).skip(1) {
        let pos = get_id(x);
        let bigger_to_the_left = fenwick_left.get(pos..n);
        let smaller_to_the_right = fenwick_right.get(0..pos);
        fenwick_left.add(pos, 1);
        fenwick_right.add(pos, -1);
        count += bigger_to_the_left as i64 * smaller_to_the_right as i64;
    }
    out_line!(count);
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
