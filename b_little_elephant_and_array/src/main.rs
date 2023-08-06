//{"name":"B. Little Elephant and Array","group":"Codeforces - Codeforces Round 136 (Div. 1)","url":"https://codeforces.com/problemset/problem/220/B","interactive":false,"timeLimit":4000,"tests":[{"input":"7 2\n3 1 2 2 3 3 7\n1 7\n3 4\n","output":"3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLittleElephantAndArray"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, Reverse};
use std::collections::{BTreeMap, HashMap};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let values: Vec<i64> = input.read_vec(n);

    let block_size = max(((n as f64).sqrt().ceil() * 1.0) as usize, 1);

    let mut queries: Vec<(usize, usize)> = input.read_vec(m);
    let mut queries: Vec<(usize, usize, usize)> = queries
        .iter_mut()
        .enumerate()
        .map(|(i, (l, r))| (*l - 1, *r - 1, i))
        .collect_vec();
    queries.sort_unstable_by(|&(l1, r1, _), &(l2, r2, _)| {
        let id1 = l1 / block_size;
        let id2 = l2 / block_size;
        (id2, r2).cmp(&(id1, r1))
    });

    let mut answer = 0;
    let mut count: BTreeMap<i64, usize> = BTreeMap::new();

    let mut cur_l = 0;
    let mut cur_r = 0;
    let c = count.entry(values[0]).or_default();
    if *c == values[0] as usize {
        answer -= 1;
    }
    *c += 1;
    if *c == values[0] as usize {
        answer += 1;
    }

    let mut answers = vec![0; m];
    for (l, r, id) in queries {
        let mut add = |x| {
            let x = values[x];
            let c = count.entry(x).or_default();
            if *c == x as usize {
                answer -= 1;
            }
            *c += 1;
            if *c == x as usize {
                answer += 1;
            }
        };

        while (cur_l > l) {
            cur_l -= 1;
            add(cur_l);
        }
        while (cur_r < r) {
            cur_r += 1;
            add(cur_r);
        }

        let mut remove = |x| {
            let x = values[x];
            let c = count.entry(x).or_default();
            if *c == x as usize {
                answer -= 1;
            }
            *c -= 1;

            if *c == x as usize {
                answer += 1;
            }
        };

        while (cur_l < l) {
            remove(cur_l);
            cur_l += 1;
        }
        while (cur_r > r) {
            remove(cur_r);
            cur_r -= 1;
        }
        answers[id] = answer;
    }

    out_line!(answers);
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
