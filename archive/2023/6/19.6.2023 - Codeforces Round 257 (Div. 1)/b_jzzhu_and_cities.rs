//{"name":"B. Jzzhu and Cities","group":"Codeforces - Codeforces Round 257 (Div. 1)","url":"https://codeforces.com/problemset/problem/449/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5 3\n1 2 1\n2 3 2\n1 3 3\n3 4 4\n1 5 5\n3 5\n4 5\n5 5\n","output":"2\n"},{"input":"2 2 3\n1 2 2\n2 1 3\n2 1\n2 2\n2 3\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BJzzhuAndCities"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let mut adj = vec![vec![]; n + 1];

    (0..m).for_each(|_| {
        let u: usize = input.read_size();
        let v: usize = input.read_size();
        let x: i64 = input.read_long();

        adj[u].push((v, x));
        adj[v].push((u, x));
    });

    let trains = (0..k)
        .map(|_| {
            let x: usize = input.read_size();
            let c: i64 = input.read_long();

            (x, c)
        })
        .collect::<Vec<_>>();

    for &(x, c) in &trains {
        adj[1].push((x, c));
    }

    // Dijkstra
    const INF: i64 = i64::MAX;
    let mut min_dist = vec![INF; n + 1];
    let mut processed = vec![false; n + 1];
    let mut count = vec![0_i128; n + 1];
    count[1] = 1;
    // (dist, target)
    min_dist[1] = 0;
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((Reverse(0), 1));
    while !priority_queue.is_empty() {
        let (Reverse(dist), current) = priority_queue.pop().unwrap();
        if processed[current] {
            continue;
        }
        processed[current] = true;

        for &(neighbor, weight) in &adj[current] {
            // relax
            if dist + weight < min_dist[neighbor] {
                count[neighbor] = count[current];
                min_dist[neighbor] = dist + weight;
                priority_queue.push((Reverse(min_dist[neighbor]), neighbor));
            } else if dist + weight == min_dist[neighbor] {
                count[neighbor] += count[current];
            }
        }
    }
    count[1] = 0;

    let mut answer = 0;
    // remove the bad ones
    for (node, cost) in trains {
        assert!(cost >= min_dist[node]);
        if cost > min_dist[node] {
            answer += 1;
        } else if count[node] > 1 {
            answer += 1;
            count[node] -= 1;
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
