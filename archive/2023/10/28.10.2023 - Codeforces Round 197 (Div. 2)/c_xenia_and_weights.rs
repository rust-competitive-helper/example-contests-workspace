//{"name":"C. Xenia and Weights","group":"Codeforces - Codeforces Round 197 (Div. 2)","url":"https://codeforces.com/problemset/problem/339/C","interactive":false,"timeLimit":2000,"tests":[{"input":"0000000101\n3\n","output":"YES\n8 10 8\n"},{"input":"1000000000\n2\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CXeniaAndWeights"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let weights = input
        .read_string()
        .chars()
        .map(|x| if x == '1' { 1 } else { 0 })
        .collect_vec();
    let m = input.read_size();

    let mut dp = vec![vec![vec![false; 11]; 11]; m + 1];
    let mut parent = vec![vec![vec![None; 11]; 11]; m + 1];
    for weight in 1..=10 {
        if weights[weight - 1] == 0 {
            continue;
        }
        dp[1][weight][weight] = true;
    }

    for weights_already in 1..m {
        for last_weight in 1..=10 {
            for weight_difference in 1..=10 {
                if dp[weights_already][last_weight][weight_difference] == false {
                    continue;
                }

                for next_weight in 1..=10 {
                    if weights[next_weight - 1] == 0 {
                        continue;
                    }
                    if next_weight <= weight_difference {
                        continue;
                    }
                    if next_weight == last_weight {
                        continue;
                    }

                    dp[weights_already + 1][next_weight][next_weight - weight_difference] = true;
                    parent[weights_already + 1][next_weight][next_weight - weight_difference] =
                        Some(last_weight);
                }
            }
        }
    }

    for last_weight in 1..=10 {
        for weight_difference in 1..=10 {
            if dp[m][last_weight][weight_difference] {
                let mut current_m = m;
                let mut current_last_weight = last_weight;
                let mut weights = vec![last_weight];
                let mut current_weight_difference = weight_difference;
                while let Some(w) =
                    parent[current_m][current_last_weight][current_weight_difference]
                {
                    weights.push(w);
                    current_m -= 1;
                    current_weight_difference = current_last_weight - current_weight_difference;
                    current_last_weight = w;
                }

                weights.reverse();

                out_line!("YES");
                out_line!(weights);
                return;
            }
        }
    }
    out_line!("NO");
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
