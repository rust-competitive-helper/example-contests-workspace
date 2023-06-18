//{"name":"D. Roman and Numbers","group":"Codeforces - Codeforces Round 235 (Div. 2)","url":"https://codeforces.com/problemset/problem/401/D","interactive":false,"timeLimit":4000,"tests":[{"input":"104 2\n","output":"3\n"},{"input":"223 4\n","output":"1\n"},{"input":"7067678 8\n","output":"47\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRomanAndNumbers"}}}

use algo_lib::collections::array2d::Array2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();
    let m = input.read_long();

    let digits = {
        let mut n = n;
        let mut d = vec![];
        while n != 0 {
            d.push(n % 10);
            n /= 10;
        }
        d
    };
    let num_digits = digits.len();

    let mut dp = Array2d::new(1 << num_digits, (m + 1) as usize, 0_i64);
    dp[0][0] = 1;
    for bitmask in 0_u32..(1 << num_digits) {
        let already_picked = bitmask.count_ones();
        let next_power = 10_i64.pow(already_picked) % m;
        for remainder in 0..m {
            for b in 0..(num_digits) {
                if (bitmask >> b) & 1 == 1 {
                    // Already picked this digit.
                } else {
                    let picking_last = already_picked == (num_digits - 1) as u32;
                    if picking_last && digits[b] == 0 {
                        continue;
                    }
                    // Picking this digit.
                    let this_remainder = (digits[b] * next_power) % m;
                    let bitmask_with_this_bit = bitmask + (1 << b);
                    let new_remainder = (this_remainder + remainder) % m;
                    dp[bitmask_with_this_bit as usize][new_remainder as usize] +=
                        dp[bitmask as usize][remainder as usize];
                }
            }
        }
    }

    let factorial = {
        let mut v = vec![1];
        for i in 1..=18 {
            v.push(v[i - 1] * (i as i64));
        }
        v
    };

    let full_mask = (1 << num_digits) - 1;
    let mut total = dp[full_mask][0];
    let different_digits = {
        let mut map: BTreeMap<i64, i64> = BTreeMap::new();
        for d in digits {
            *map.entry(d).or_default() += 1;
        }
        map
    };
    for (_d, c) in different_digits {
        total /= factorial[c as usize];
    }
    out_line!(total);
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
