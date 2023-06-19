//{"name":"C. Lucky Tree","group":"Codeforces - Codeforces Beta Round 84 (Div. 1 Only)","url":"https://codeforces.com/problemset/problem/109/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2 4\n3 1 2\n1 4 7\n","output":"16\n"},{"input":"4\n1 2 4\n1 3 47\n1 4 7447\n","output":"24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLuckyTree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};
use std::cmp::max;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read_size();

    let is_lucky = |mut num: i32| {
        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        !digits.iter().any(|&d| d != 4 && d != 7)
    };

    let mut adj = vec![vec![]; n + 1];
    (0..(n - 1)).for_each(|_| {
        let a: usize = input.read_size();
        let b: usize = input.read_size();
        let w: i32 = input.read_int();

        adj[a].push((b, is_lucky(w)));
        adj[b].push((a, is_lucky(w)));
    });

    let mut subtree_size = vec![1; n + 1];
    let mut compute_subtree_size = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &(neighbor, _) in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);

            subtree_size[current] += subtree_size[neighbor];
        }
    });
    compute_subtree_size.call(1, 0);

    let mut lucky_nodes_in = vec![0; n + 1];
    let mut compute_lucky_nodes_in = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &(neighbor, lucky) in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);

            if lucky {
                lucky_nodes_in[current] += subtree_size[neighbor];
            } else {
                lucky_nodes_in[current] += lucky_nodes_in[neighbor];
            }
        }
    });
    compute_lucky_nodes_in.call(1, 0);

    let mut lucky_nodes_out = vec![0; n + 1];
    let mut compute_lucky_nodes_out =
        RecursiveFunction2::new(|f, current: usize, parent: usize| {
            for &(neighbor, lucky) in &adj[current] {
                if neighbor == parent {
                    continue;
                }

                if lucky {
                    lucky_nodes_out[neighbor] = n as i64 - subtree_size[neighbor];
                } else {
                    lucky_nodes_out[neighbor] = lucky_nodes_out[current] + lucky_nodes_in[current]
                        - lucky_nodes_in[neighbor];
                }

                f.call(neighbor, current);
            }
        });
    compute_lucky_nodes_out.call(1, 0);

    let mut total_lucky_triplets: i64 = 0;

    let mut count_lucky_triplets = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        // choose from in
        total_lucky_triplets += max(0, lucky_nodes_in[current] * (lucky_nodes_in[current] - 1));

        // choose from out
        total_lucky_triplets += max(0, lucky_nodes_out[current] * (lucky_nodes_out[current] - 1));

        // from both
        total_lucky_triplets += max(0, lucky_nodes_in[current] * lucky_nodes_out[current] * 2);

        for &(neighbor, _) in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);
        }
    });
    count_lucky_triplets.call(1, 0);

    out_line!(total_lucky_triplets);
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
