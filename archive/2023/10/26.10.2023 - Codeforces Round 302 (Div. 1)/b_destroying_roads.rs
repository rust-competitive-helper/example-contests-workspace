//{"name":"B. Destroying Roads","group":"Codeforces - Codeforces Round 302 (Div. 1)","url":"https://codeforces.com/problemset/problem/543/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n1 2\n2 3\n3 4\n4 5\n1 3 2\n3 5 2\n","output":"0\n"},{"input":"5 4\n1 2\n2 3\n3 4\n4 5\n1 3 2\n2 4 2\n","output":"1\n"},{"input":"5 4\n1 2\n2 3\n3 4\n4 5\n1 3 2\n3 5 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDestroyingRoads"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let mut graph = Graph::new(n);
    for _ in 0..m {
        let (a, b) = (input.read_size() - 1, input.read_size() - 1);
        graph.add_edge(a, b);
        graph.add_edge(b, a);
    }

    let (s1, t1, l1) = (
        input.read_size() - 1,
        input.read_size() - 1,
        input.read_size(),
    );
    let (s2, t2, l2) = (
        input.read_size() - 1,
        input.read_size() - 1,
        input.read_size(),
    );
    let mut dist = vec![vec![]; n];
    for v in 0..n {
        dist[v] = graph
            .breadth_first_search(v)
            .into_iter()
            .map(|d| d.unwrap())
            .collect();
    }

    // If the paths do not intersect, the best answer is both of the shortest paths
    let shortest_path_1 = dist[s1][t1];
    let shortest_path_2 = dist[s2][t2];
    if shortest_path_1 > l1 {
        out_line!(-1);
        return;
    }
    if shortest_path_2 > l2 {
        out_line!(-1);
        return;
    }
    let non_intersecting = if shortest_path_1 + shortest_path_2 <= m {
        m - (shortest_path_1 + shortest_path_2)
    } else {
        0
    };

    let mut answer = non_intersecting;
    for left_endpoint in 0..n {
        for right_endpoint in 0..n {
            let middle = dist[left_endpoint][right_endpoint];

            let path_1 = dist[s1][left_endpoint] + middle + dist[right_endpoint][t1];
            let path_2 = dist[s2][left_endpoint] + middle + dist[right_endpoint][t2];
            if path_1 <= l1 && path_2 <= l2 {
                answer.maxim(m.saturating_sub(path_1 + path_2 - middle));
            }

            let path_1 = dist[s1][left_endpoint] + middle + dist[right_endpoint][t1];
            let path_2 = dist[t2][left_endpoint] + middle + dist[right_endpoint][s2];
            if path_1 <= l1 && path_2 <= l2 {
                answer.maxim(m.saturating_sub(path_1 + path_2 - middle));
            }
            let middle = dist[left_endpoint][right_endpoint];
            let left = dist[s1][left_endpoint] + dist[s2][left_endpoint];
            let right = dist[t1][right_endpoint] + dist[t2][right_endpoint];
            if dist[s1][left_endpoint] + middle + dist[right_endpoint][t1] <= l1
                && dist[s2][left_endpoint] + middle + dist[right_endpoint][t2] <= l2
            {
                answer.maxim(m.saturating_sub(middle + left + right));
            }

            let left = dist[s1][left_endpoint] + dist[t2][left_endpoint];
            let right = dist[t1][right_endpoint] + dist[s2][right_endpoint];
            if dist[s1][left_endpoint] + middle + dist[right_endpoint][t1] <= l1
                && dist[s2][left_endpoint] + middle + dist[right_endpoint][t2] <= l2
            {
                answer.maxim(m.saturating_sub(middle + left + right));
            }
        }
    }
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
