//{"name":"D. Count Good Substrings","group":"Codeforces - Codeforces Round 258 (Div. 2)","url":"https://codeforces.com/problemset/problem/451/D","interactive":false,"timeLimit":2000,"tests":[{"input":"bb\n","output":"1 2\n"},{"input":"baab\n","output":"2 4\n"},{"input":"babb\n","output":"2 5\n"},{"input":"babaa\n","output":"2 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCountGoodSubstrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::ops::{Add, Sub};
use std::thread::current;

#[derive(Default, Copy, Clone)]
struct State {
    reachable_even: i64,
    reachable_odd: i64,
    reachable_even_odd_values: i64,
    reachable_odd_odd_values: i64,
}

impl Add for State {
    type Output = State;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            reachable_even: self.reachable_even + rhs.reachable_even,
            reachable_odd: self.reachable_odd + rhs.reachable_odd,
            reachable_even_odd_values: self.reachable_even_odd_values
                + rhs.reachable_even_odd_values,
            reachable_odd_odd_values: self.reachable_odd_odd_values + rhs.reachable_odd_odd_values,
        }
    }
}
impl Sub for State {
    type Output = State;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            reachable_even: self.reachable_even - rhs.reachable_even,
            reachable_odd: self.reachable_odd - rhs.reachable_odd,
            reachable_even_odd_values: self.reachable_even_odd_values
                - rhs.reachable_even_odd_values,
            reachable_odd_odd_values: self.reachable_odd_odd_values - rhs.reachable_odd_odd_values,
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = {
        let str = input.read_string();
        str.chars().collect::<Vec<char>>()
    };

    // 1. Compress consecutive characters
    let compressed = {
        let mut last_char = s[0];
        let mut current_count = 1;
        let mut compressed = vec![];
        for &c in s.iter().skip(1) {
            if c == last_char {
                current_count += 1;
            } else {
                compressed.push((last_char, current_count));
                last_char = c;
                current_count = 1;
            }
        }
        compressed.push((last_char, current_count));
        compressed
    };
    let n = compressed.len();

    // 2. Compute reachable with even/odd
    let mut current_length = 0;
    let mut state_a = vec![State::default(); n];
    let mut state_b = vec![State::default(); n];
    for (i, &(character, count)) in compressed.iter().enumerate() {
        if character == 'a' {
            if current_length % 2 == 0 {
                state_a[i].reachable_even += count;
                if count % 2 == 1 {
                    state_a[i].reachable_even_odd_values += 1;
                }
            } else {
                state_a[i].reachable_odd += count;
                if count % 2 == 1 {
                    state_a[i].reachable_odd_odd_values += 1;
                }
            }
        } else {
            assert_eq!(character, 'b');
            if current_length % 2 == 0 {
                state_b[i].reachable_even += count;
                if count % 2 == 1 {
                    state_b[i].reachable_even_odd_values += 1;
                }
            } else {
                state_b[i].reachable_odd += count;
                if count % 2 == 1 {
                    state_b[i].reachable_odd_odd_values += 1;
                }
            }
        }

        current_length += count;
    }

    for i in 1..n {
        state_a[i] = state_a[i - 1] + state_a[i];
        state_b[i] = state_b[i - 1] + state_b[i];
    }

    let state_a_for_suffix = |start: usize| {
        if start == 0 {
            state_a[n - 1]
        } else {
            state_a[n - 1] - state_a[start - 1]
        }
    };

    let state_b_for_suffix = |start: usize| {
        if start == 0 {
            state_b[n - 1]
        } else {
            state_b[n - 1] - state_b[start - 1]
        }
    };

    let mut answer_even = 0;
    let mut answer_odd = 0;

    // Solve for only one letter
    for &(_, count) in &compressed {
        for size in 1..=count {
            if size % 2 == 0 {
                answer_even += count - size + 1;
            } else {
                answer_odd += count - size + 1;
            }
        }
    }
    let mut current_length = 0;
    for (i, &(character, count)) in compressed.iter().enumerate() {
        if i + 2 >= n {
            break;
        }
        let inverted = (current_length + count) % 2 == 1;

        if character == 'a' {
            let state = state_a_for_suffix(i + 1);
            // Handle even
            if count % 2 == 0 {
                // Always half/half
                let total = count * (state.reachable_even + state.reachable_odd);
                answer_even += total / 2;
                answer_odd += total / 2;
            } else {
                // We have one more. Now we must know if this one is even or odd
                // One more even
                {
                    // One more even
                    if inverted {
                        let total = count * state.reachable_odd - state.reachable_odd_odd_values;
                        answer_even += state.reachable_odd_odd_values;
                        answer_even += total / 2;
                        answer_odd += total / 2;
                    } else {
                        let total = count * state.reachable_even - state.reachable_even_odd_values;
                        answer_even += state.reachable_even_odd_values;
                        answer_even += total / 2;
                        answer_odd += total / 2;
                    }
                }

                {
                    // One more odd
                    if inverted {
                        let total = count * state.reachable_even - state.reachable_even_odd_values;
                        answer_odd += state.reachable_even_odd_values;
                        answer_odd += total / 2;
                        answer_even += total / 2;
                    } else {
                        let total = count * state.reachable_odd - state.reachable_odd_odd_values;
                        answer_odd += state.reachable_odd_odd_values;
                        answer_odd += total / 2;
                        answer_even += total / 2;
                    }
                }
            }
        } else {
            let state = state_b_for_suffix(i + 1);
            // Handle even
            if count % 2 == 0 {
                // It does not matter the parity we reach. We will always have half even and half odd.
                let total = count * (state.reachable_even + state.reachable_odd);
                answer_even += total / 2;
                answer_odd += total / 2;
            } else {
                // We have one more. Now we must know if this one is even or odd
                // One more even
                {
                    // One more even
                    if inverted {
                        let total = count * state.reachable_odd - state.reachable_odd_odd_values;
                        answer_even += state.reachable_odd_odd_values;
                        answer_even += total / 2;
                        answer_odd += total / 2;
                    } else {
                        let total = count * state.reachable_even - state.reachable_even_odd_values;
                        answer_even += state.reachable_even_odd_values;
                        answer_even += total / 2;
                        answer_odd += total / 2;
                    }
                }

                {
                    // One more odd
                    if inverted {
                        let total = count * state.reachable_even - state.reachable_even_odd_values;
                        answer_odd += state.reachable_even_odd_values;
                        answer_odd += total / 2;
                        answer_even += total / 2;
                    } else {
                        let total = count * state.reachable_odd - state.reachable_odd_odd_values;
                        answer_odd += state.reachable_odd_odd_values;
                        answer_odd += total / 2;
                        answer_even += total / 2;
                    }
                }
            }
        }
        current_length += count;
    }
    out_line!(answer_even);
    out_line!(answer_odd);
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
