//{"name":"E. Breaking Good","group":"Codeforces - Codeforces Round 287 (Div. 2)","url":"https://codeforces.com/problemset/problem/507/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1\n1 2 0\n","output":"1\n1 2 1\n"},{"input":"4 4\n1 2 1\n1 3 0\n2 3 1\n3 4 1\n","output":"3\n1 2 0\n1 3 1\n2 3 0\n"},{"input":"8 9\n1 2 0\n8 3 0\n2 3 1\n1 4 1\n8 7 0\n1 5 1\n4 6 1\n5 7 0\n6 8 0\n","output":"3\n2 3 0\n1 5 0\n6 8 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBreakingGood"}}}

use algo_lib::collections::array2d::Array2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::{HashMap, HashSet, VecDeque};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let edges: Vec<(usize, usize, usize)> = input.read_vec(m);

    let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; n + 1];
    for &(a, b, good) in &edges {
        adj[a].push((b, good));
        adj[b].push((a, good));
    }

    let total_ones = edges.iter().filter(|(_, _, g)| *g == 1).count() as i64;

    let mut queue = VecDeque::new();
    let mut min_dist = vec![i64::MAX; n + 1];
    let mut min_zeros = vec![i64::MAX; n + 1];
    let mut parent = vec![0; n + 1];
    min_dist[1] = 0;
    queue.push_back((1, 0, 0));
    while let Some((current_node, current_dist, zeros)) = queue.pop_front() {
        for &(neighbor, good) in &adj[current_node] {
            let this_dist = current_dist + 1;
            let zeros = zeros + if good == 1 { 0 } else { 1 };
            if this_dist > min_dist[neighbor] {
                continue;
            }

            if this_dist < min_dist[neighbor] {
                min_dist[neighbor] = this_dist;
                min_zeros[neighbor] = zeros;
                parent[neighbor] = current_node;
            } else {
                if min_zeros[neighbor].minim(zeros) {
                    parent[neighbor] = current_node;
                };
            }
            queue.push_back((neighbor, this_dist, zeros));
        }
    }

    let min_ones = min_dist[n] - min_zeros[n];
    let affected = min_zeros[n] + total_ones - min_ones;
    out_line!(affected);

    let mut in_path_edges = HashSet::new();
    let mut current_node = n;
    loop {
        if parent[current_node] == 0 {
            break;
        }
        in_path_edges.insert((current_node, parent[current_node]));
        in_path_edges.insert((parent[current_node], current_node));
        current_node = parent[current_node];
    }

    for (a, b, g) in edges {
        if in_path_edges.contains(&(a, b)) {
            if g == 0 {
                out_line!(a, b, 1);
            }
        } else {
            if g == 1 {
                out_line!(a, b, 0);
            }
        }
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
