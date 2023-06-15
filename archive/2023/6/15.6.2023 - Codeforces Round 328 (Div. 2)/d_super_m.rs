//{"name":"D. Super M","group":"Codeforces - Codeforces Round 328 (Div. 2)","url":"https://codeforces.com/problemset/problem/592/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7 2\n1 2\n1 3\n1 4\n3 5\n3 6\n3 7\n2 7\n","output":"2\n3\n"},{"input":"6 4\n1 2\n2 3\n2 4\n4 5\n4 6\n2 4 5 6\n","output":"2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSuperM"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};
use std::cmp::min;

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let mut adj = vec![vec![]; n + 1];
    let mut is_marked = vec![false; n + 1];
    for _ in 0..(n - 1) {
        let (a, b) = (input.read_size(), input.read_size());
        adj[a].push(b);
        adj[b].push(a);
    }
    for _ in 0..m {
        let x = input.read_size();
        is_marked[x] = true;
    }

    let mut in_start_and_finish = vec![None; n + 1];
    let mut in_only_finish = vec![None; n + 1];
    let mut compute_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        let children = adj[current].iter().filter(|&&x| x != parent);
        children.clone().for_each(|c| f.call(*c, current));

        if is_marked[current] {
            in_start_and_finish[current] = Some(0);
            in_only_finish[current] = Some(0);
        }

        let no_marked_below = children.clone().all(|&x| in_start_and_finish[x].is_none());
        if no_marked_below {
            return;
        }

        let all_start_and_finish = children
            .clone()
            .filter(|&&x| in_start_and_finish[x].is_some());
        let sum_start_and_finish = all_start_and_finish
            .map(|&x| in_start_and_finish[x].unwrap() + 2)
            .sum::<i64>();
        in_start_and_finish[current] = Some(sum_start_and_finish);

        let all_finish = children.clone().filter(|&&x| in_only_finish[x].is_some());
        let best_pick = all_finish
            .map(|&candidate| {
                assert!(in_start_and_finish[candidate].is_some());

                let this_contribution = 2 + in_start_and_finish[candidate].unwrap();
                let others = sum_start_and_finish - this_contribution;
                1 + in_only_finish[candidate].unwrap() + others
            })
            .min();
        assert!(best_pick.is_some());
        in_only_finish[current] = best_pick;
    });
    compute_subtree.call(1, 0);

    let mut out_start_and_finish = vec![None; n + 1];
    let mut out_only_finish = vec![None; n + 1];
    let mut compute_out_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        if is_marked[current] {
            if out_start_and_finish[current].is_none() {
                out_start_and_finish[current] = Some(0);
            }
            if out_only_finish[current].is_none() {
                out_only_finish[current] = Some(0);
            }
        }
        let sum_of_start_and_finish_from_children = {
            let mut res = 0;
            for &child in &adj[current] {
                if child != parent {
                    if let Some(v) = in_start_and_finish[child] {
                        res += 2 + v;
                    }
                }
            }
            if res != 0 {
                Some(res)
            } else {
                None
            }
        };

        // Compute out_start_and_finish for each children.
        for &child in &adj[current] {
            if child == parent {
                continue;
            }
            // We have previously computed the sum for all children.
            // But we must not count this child twice, so we must remove its contribution.
            let sum_from_others = {
                if let Some(sum) = sum_of_start_and_finish_from_children {
                    if let Some(mine) = in_start_and_finish[child] {
                        let my_contribution = mine + 2;
                        Some(sum - my_contribution)
                    } else {
                        Some(sum)
                    }
                } else {
                    None
                }
            };

            if let Some(calculated_for_parent) = out_start_and_finish[current] {
                out_start_and_finish[child] = if let Some(sum) = sum_from_others {
                    Some(calculated_for_parent + sum + 2)
                } else {
                    None
                };
            } else {
                // Parent does not contribute with anything.
                out_start_and_finish[child] = if let Some(sum) = sum_from_others {
                    if sum == 0 {
                        None
                    } else {
                        Some(sum + 2)
                    }
                } else {
                    None
                };
            }
        }

        // Compute only finish for each children

        // We have two choices:
        // 1. We choose to start at out-subtree of parent
        //      - We must use out_only_finish[current] and in_start_and_finish for siblings.
        // 2. We choose to start at one sibling.
        //      - We must use out_start_and_finish[current], only_finish for the particular sibling
        //        and in_start_and_finish for the others.

        for &child in &adj[current] {
            if child == parent {
                continue;
            }
            // Choose to start at out-subtree of current.
            let to_use_from_parent = out_only_finish[current];
            let sum_start_and_finish_from_others = {
                if let Some(sum) = sum_of_start_and_finish_from_children {
                    if let Some(mine) = in_start_and_finish[child] {
                        let my_contribution = mine + 2;
                        Some(sum - my_contribution)
                    } else {
                        Some(sum)
                    }
                } else {
                    None
                }
            };
            let to_use_from_siblings = sum_start_and_finish_from_others;
            let mut res = 0;
            if let Some(v) = to_use_from_parent {
                res += v;
            }
            if let Some(v) = to_use_from_siblings {
                res += v;
            }
            if res != 0 || is_marked[current] {
                res += 1; // Get to child.
                out_only_finish[child] = Some(res);
            }
        }

        let children_contribution = {
            let mut contributions = vec![];
            for &child in &adj[current] {
                if child == parent {
                    continue;
                }

                if let Some(v) = in_start_and_finish[child] {
                    assert!(in_only_finish[child].is_some());
                    assert!(sum_of_start_and_finish_from_children.is_some());

                    let this_contribution = -(in_start_and_finish[child].unwrap() + 2)
                        + (in_only_finish[child].unwrap() + 1);
                    contributions.push(this_contribution);
                }
            }

            contributions.sort_unstable();
            contributions
        };

        for &child in &adj[current] {
            if child == parent {
                continue;
            }
            // Choose to start at a sibling.
            let to_use_from_parent = out_start_and_finish[current];
            let sum_start_and_finish_from_others = {
                if let Some(sum) = sum_of_start_and_finish_from_children {
                    if let Some(mine) = in_start_and_finish[child] {
                        let my_contribution = mine + 2;
                        Some(sum - my_contribution)
                    } else {
                        Some(sum)
                    }
                } else {
                    None
                }
            };
            let to_use_from_siblings = {
                let this_contribution = if let Some(v) = in_start_and_finish[child] {
                    assert!(in_only_finish[child].is_some());
                    assert!(sum_start_and_finish_from_others.is_some());

                    let this_contribution = -(in_start_and_finish[child].unwrap() + 2)
                        + (in_only_finish[child].unwrap() + 1);
                    Some(this_contribution)
                } else {
                    None
                };

                if children_contribution.is_empty() {
                    None
                } else if this_contribution.is_none() {
                    Some(sum_start_and_finish_from_others.unwrap() + children_contribution[0])
                } else {
                    let this_contribution = this_contribution.unwrap();
                    if this_contribution == children_contribution[0] {
                        if children_contribution.len() == 1 {
                            // Some(this_contribution)
                            None
                        } else {
                            Some(
                                // sum_of_start_and_finish_from_children.unwrap()
                                //     + children_contribution[1],
                                sum_start_and_finish_from_others.unwrap()
                                    + children_contribution[1], // + this_contribution,
                            )
                        }
                    } else {
                        Some(
                            // sum_of_start_and_finish_from_children.unwrap()
                            //     + children_contribution[0],
                            sum_start_and_finish_from_others.unwrap() + children_contribution[0], // + this_contribution,
                        )
                    }
                }
            };
            let mut res = 0;
            if let Some(v) = to_use_from_parent {
                res += v;
            }

            if let Some(v) = to_use_from_siblings {
                res += v;
            }
            if res != 0 {
                res += 1; // Get to child.
                if let Some(v) = out_only_finish[child] {
                    out_only_finish[child] = Some(min(v, res));
                } else {
                    out_only_finish[child] = Some(res);
                }
            }
        }

        for &child in &adj[current] {
            if child == parent {
                continue;
            }
            f.call(child, current);
        }
    });
    compute_out_subtree.call(1, 0);

    let mut answer = i64::MAX;
    for v in 1..=n {
        answer = min(
            answer,
            in_start_and_finish[v]
                .unwrap_or(i64::MAX)
                .saturating_add(out_only_finish[v].unwrap_or(i64::MAX)),
        );
        answer = min(
            answer,
            in_only_finish[v]
                .unwrap_or(i64::MAX)
                .saturating_add(out_start_and_finish[v].unwrap_or(i64::MAX)),
        );
    }

    for v in 1..=n {
        let answer1 = in_start_and_finish[v]
            .unwrap_or(i64::MAX)
            .saturating_add(out_only_finish[v].unwrap_or(i64::MAX));
        let answer2 = in_only_finish[v]
            .unwrap_or(i64::MAX)
            .saturating_add(out_start_and_finish[v].unwrap_or(i64::MAX));

        if min(answer1, answer2) == answer {
            out_line!(v);
            out_line!(answer);
            break;
        }
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
