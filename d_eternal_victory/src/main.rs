//{"name":"D. Eternal Victory","group":"Codeforces - Codeforces Beta Round 57 (Div. 2)","url":"https://codeforces.com/problemset/problem/61/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 3\n2 3 4\n","output":"7\n"},{"input":"3\n1 2 3\n1 3 3\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DEternalVictory"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction, RecursiveFunction2};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let edges: Vec<(usize, usize, i64)> = input.read_vec(n - 1);

    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n + 1];
    edges.iter().for_each(|&(a, b, w)| {
        adj[a].push((b, w));
        adj[b].push((a, w));
    });

    let mut last_weight = vec![0; n + 1];
    let mut dist_from_root = vec![0; n + 1];

    let mut cost_subtree = vec![0; n + 1];
    let mut solve_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        let mut cost_from_children = 0;
        for &(child, weight) in &adj[current] {
            if child == parent {
                continue;
            }
            last_weight[child] = weight;
            dist_from_root[child] = dist_from_root[current] + weight;
            f.call(child, current);

            cost_from_children += weight + cost_subtree[child];
        }
        cost_subtree[current] = cost_from_children;
    });
    solve_subtree.call(1, 0);

    let mut cost_out = vec![0; n + 1];
    let mut solve_out = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        let mut cost_from_children = 0;
        for &(child, weight) in &adj[current] {
            if child == parent {
                continue;
            }
            cost_from_children += weight + cost_subtree[child];
        }

        for &(child, weight) in &adj[current] {
            if child == parent {
                continue;
            }
            let cost_from_this_child = weight + cost_subtree[child];
            let cost_from_other_children = cost_from_children - cost_from_this_child;

            cost_out[child] = cost_out[current] + cost_from_other_children + weight;
        }

        for &(child, weight) in &adj[current] {
            if child == parent {
                continue;
            }
            f.call(child, current);
        }
    });
    solve_out.call(1, 0);

    let answer = (1..=n)
        .map(|x| 2 * cost_subtree[x] + 2 * cost_out[x] - dist_from_root[x])
        .min();
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
