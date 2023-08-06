//{"name":"D. Prefixes and Suffixes","group":"Codeforces - Codeforces Round 246 (Div. 2)","url":"https://codeforces.com/problemset/problem/432/D","interactive":false,"timeLimit":1000,"tests":[{"input":"ABACABA\n","output":"3\n1 4\n3 2\n7 1\n"},{"input":"AAA\n","output":"3\n1 3\n2 2\n3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrefixesAndSuffixes"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string_algorithms::StringAlgorithms;
use algo_lib::{out, out_line};
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read_string();

    let p = s.as_bytes().prefix_function();

    let mut size = *p.last().unwrap();
    if size == s.len() - 1 {
        size += 1;
    }
    let last_suffix = &s[..size];
    let mut is_good = vec![false; s.len() + 1];
    let mut current_size = s.len();
    while current_size > 0 {
        is_good[current_size] = true;
        current_size = p[current_size - 1];
    }
    let good_prefixes = is_good.iter().filter(|x| **x).count();

    let new_string = String::from(last_suffix) + "#" + &s;
    let new_p = new_string.as_bytes().prefix_function();

    let mut count = HashMap::new();
    for v in new_p.into_iter().skip(size) {
        *count.entry(v).or_default() += 1;
    }

    let mut answers = vec![];
    out_line!(good_prefixes);
    for length in (1..=s.len()).rev() {
        let c = *count.get(&length).unwrap_or(&0);

        let next = p[length - 1];
        *count.entry(next).or_default() += c;

        if is_good[length] {
            answers.push((length, c));
        }
    }
    answers.sort_unstable();
    for (l, c) in answers {
        if l == s.len() {
            out_line!(l, 1);
        } else {
            out_line!(l, c);
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
