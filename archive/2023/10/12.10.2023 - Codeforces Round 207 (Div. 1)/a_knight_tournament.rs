//{"name":"A. Knight Tournament","group":"Codeforces - Codeforces Round 207 (Div. 1)","url":"https://codeforces.com/problemset/problem/356/A","interactive":false,"timeLimit":3000,"tests":[{"input":"4 3\n1 2 1\n1 3 3\n1 4 4\n","output":"3 1 4 0\n"},{"input":"8 4\n3 5 4\n3 7 6\n2 8 8\n1 8 1\n","output":"0 8 4 6 4 8 6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKnightTournament"}}}

use std::collections::BTreeSet;

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let mut conquered_by = vec![0; n + 1];
    let mut alive: BTreeSet<usize> = BTreeSet::from_iter(1..=n);
    for fight in 0..m {
        let (lower, upper) = (input.read_size(), input.read_size());
        let winner = input.read_size();

        let to_remove = alive
            .range(lower..=upper)
            .map(|x| *x)
            .into_iter()
            .collect_vec();
        for dead in to_remove.clone() {
            conquered_by[dead] = winner;
        }
        for x in to_remove {
            alive.remove(&x);
        }

        alive.insert(winner);
    }

    for i in 1..=n {
        if conquered_by[i] == i {
            out!(0);
        } else {
            out!(conquered_by[i]);
        }
        out!(" ");
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
