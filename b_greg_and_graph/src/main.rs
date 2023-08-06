//{"name":"B. Greg and Graph","group":"Codeforces - Codeforces Round 179 (Div. 1)","url":"https://codeforces.com/problemset/problem/295/B","interactive":false,"timeLimit":3000,"tests":[{"input":"1\n0\n1\n","output":"0\n"},{"input":"2\n0 5\n4 0\n1 2\n","output":"9 0\n"},{"input":"4\n0 3 1 1\n6 0 400 1\n2 4 0 1\n1 1 1 0\n4 1 2 3\n","output":"17 23 404 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGregAndGraph"}}}

use algo_lib::collections::array2d::{Array2d, Array2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();

    let adj = input.read_long_table(n, n);

    let deleting_order = input.read_size_vec(n);

    let mut min_dist = adj.clone();

    let mut answers = vec![];
    let mut active_vertices: Vec<usize> = vec![];
    for current_vertex in deleting_order.into_iter().map(|i| i - 1).rev() {
        for &other in &active_vertices {
            min_dist[other][current_vertex] = adj[other][current_vertex];
            min_dist[current_vertex][other] = adj[current_vertex][other];
        }

        // // Ending with current.
        // for &middle in &active_vertices {
        //     for &start in &active_vertices {
        //         let using_middle =
        //             min_dist[start][middle].saturating_add(min_dist[middle][current_vertex]);
        //         min_dist[start][current_vertex].minim(using_middle);
        //     }
        // }
        //
        // // Starting with current.
        // for &middle in &active_vertices {
        //     for &end in &active_vertices {
        //         let using_middle =
        //             min_dist[current_vertex][middle].saturating_add(min_dist[middle][end]);
        //         min_dist[current_vertex][end].minim(using_middle);
        //     }
        // }

        // All paths.
        for &i in &active_vertices {
            for &j in &active_vertices {
                let using_current =
                    min_dist[i][current_vertex].saturating_add(min_dist[current_vertex][j]);
                min_dist[i][j].minim(using_current);
            }
        }

        active_vertices.push(current_vertex);
        let mut sum = 0;
        for &i in &active_vertices {
            for &j in &active_vertices {
                sum += min_dist[i][j];
            }
        }

        answers.push(sum);
    }
    answers.reverse();
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
