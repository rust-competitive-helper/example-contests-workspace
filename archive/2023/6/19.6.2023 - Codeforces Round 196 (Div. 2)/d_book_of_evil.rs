//{"name":"D. Book of Evil","group":"Codeforces - Codeforces Round 196 (Div. 2)","url":"https://codeforces.com/problemset/problem/337/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6 2 3\n1 2\n1 5\n2 3\n3 4\n4 5\n5 6\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBookOfEvil"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read_size();
    let m: usize = input.read_size();
    let d: i32 = input.read_int();

    let mut affected = vec![false; n + 1];
    (0..m).for_each(|_| {
        let x: usize = input.read_size();
        affected[x] = true;
    });

    let mut adj = vec![vec![]; n + 1];
    (1..n).for_each(|_| {
        let a: usize = input.read_size();
        let b: usize = input.read_size();

        adj[a].push(b);
        adj[b].push(a);
    });

    let mut max_distance_down: Vec<i32> = vec![-1; n + 1];
    let mut second_max_distance_down = vec![-1; n + 1];
    let mut solve_for_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for &neighbor in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);
        }

        let mut max = -1;
        let mut second_max = -1;
        for &neighbor in &adj[current] {
            if neighbor == parent {
                continue;
            }
            let dist = max_distance_down[neighbor];
            if dist != -1 {
                if dist >= max {
                    second_max = max;
                    max = dist;
                } else if dist >= second_max {
                    second_max = dist;
                }
            }
        }

        if max != -1 {
            max_distance_down[current] = max + 1;
        }

        if second_max != -1 {
            second_max_distance_down[current] = second_max + 1;
        }

        if affected[current] {
            if max_distance_down[current] == -1 {
                max_distance_down[current] = 0
            };
            if second_max_distance_down[current] == -1 {
                second_max_distance_down[current] = 0
            };
        }
    });

    solve_for_subtree.call(1, 0);

    let mut max_distance = vec![-1; n + 1];
    let mut second_max_distance = vec![-1; n + 1];
    let mut solve_for_all = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        if current == 1 {
            max_distance[current] = max_distance_down[current];
            second_max_distance[current] = second_max_distance_down[current];
        } else {
            let dist_from_parent = if max_distance[parent] != 1 + max_distance_down[current] {
                1 + max_distance[parent]
            } else if second_max_distance[parent] == -1 {
                -1
            } else {
                1 + second_max_distance[parent]
            };
            let mut possible_dists = vec![
                dist_from_parent,
                max_distance_down[current],
                second_max_distance_down[current],
            ];
            possible_dists.sort_unstable();
            max_distance[current] = possible_dists[2];
            second_max_distance[current] = possible_dists[1];
        }

        for &neighbor in &adj[current] {
            if neighbor == parent {
                continue;
            }
            f.call(neighbor, current);
        }
    });
    solve_for_all.call(1, 0);

    let mut answer = 0;
    for node in 1..=n {
        if max_distance[node] <= d {
            answer += 1;
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
