//{"name":"D. Dima and Bacteria","group":"Codeforces - Codeforces Round 234 (Div. 2)","url":"https://codeforces.com/problemset/problem/400/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 2\n1 3\n2 3 0\n3 4 0\n2 4 1\n2 1 2\n","output":"Yes\n0 2\n2 0\n"},{"input":"3 1 2\n2 1\n1 2 0\n","output":"Yes\n0 -1\n-1 0\n"},{"input":"3 2 2\n2 1\n1 2 0\n2 3 1\n","output":"Yes\n0 1\n1 0\n"},{"input":"3 0 2\n1 2\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDimaAndBacteria"}}}

use algo_lib::collections::array2d::Array2d;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::min;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m, k) = (input.read_size(), input.read_size(), input.read_size());
    let c: Vec<usize> = input.read_vec(k);
    let edges = (0..m)
        .map(|_| (input.read_size(), input.read_size(), input.read_size()))
        .collect_vec();

    let type_of = {
        let mut type_of = vec![0; n + 1];
        let mut current_bacteria = 1;
        let mut current_type = 1;
        for count in c {
            for b in 0..count {
                type_of[b + current_bacteria] = current_type;
            }
            current_bacteria += count;
            current_type += 1;
        }
        type_of
    };

    let mut dsu = DSU::new(n + 1);
    for &(a, b, w) in &edges {
        if w == 0 {
            dsu.join(a, b);
        }
    }

    let mut bacteria_by_type = vec![vec![]; k + 1];
    for b in 1..=n {
        bacteria_by_type[type_of[b]].push(b);
    }

    for t in &bacteria_by_type {
        let mut different_repr = HashSet::new();
        for b in t {
            different_repr.insert(dsu.get(*b));
        }

        if different_repr.len() > 1 {
            out_line!("No");
            return;
        }
    }

    let mut graph = Array2d::new(k + 1, k + 1, i64::MAX);
    for (a, b, w) in edges {
        let t_a = type_of[a];
        let t_b = type_of[b];
        graph[t_a][t_b] = min(graph[t_a][t_b], w as i64);
        graph[t_b][t_a] = min(graph[t_b][t_a], w as i64);
    }

    for t in 1..=k {
        graph[t][t] = 0;
    }

    let mut min_dist = graph.clone();
    for m in 1..=k {
        for t1 in 1..=k {
            for t2 in 1..=k {
                min_dist[t1][t2] = min(
                    min_dist[t1][t2],
                    min_dist[t1][m].saturating_add(min_dist[m][t2]),
                );
            }
        }
    }

    out_line!("Yes");
    for t1 in 1..=k {
        for t2 in 1..=k {
            if min_dist[t1][t2] == i64::MAX {
                out!("-1 ");
            } else {
                out!(min_dist[t1][t2]);
                out!(" ");
            }
        }
        out_line!();
    }
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
