//{"name":"C. Bear and Prime Numbers","group":"Codeforces - Codeforces Round 226 (Div. 2)","url":"https://codeforces.com/problemset/problem/385/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5 5 7 10 14 15\n3\n2 11\n3 12\n4 4\n","output":"9\n7\n0\n"},{"input":"7\n2 3 5 7 11 4 8\n2\n8 10\n2 123\n","output":"0\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBearAndPrimeNumbers"}}}

use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const MAX_SIZE: usize = 10_000_005;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let values = input.read_u64_vec(n);

    let frequency = {
        let mut res = vec![0_usize; MAX_SIZE];
        for x in values {
            res[x as usize] += 1;
        }
        res
    };

    let mut multiples_in_sequence = vec![0; MAX_SIZE];
    let mut all_primes = vec![];
    // Sieve of Erathostenes
    {
        let left = 2;
        let right = MAX_SIZE;
        let mut is_prime = vec![true; right];
        is_prime[1] = false;
        for value in left..right {
            if !is_prime[value] {
                continue;
            }

            let prime = value;
            all_primes.push(prime);
            multiples_in_sequence[prime] = frequency[prime];
            // In the standard Sieve,
            // We can start at prime * prime because any smaller multiple was already considered.
            // Example: if `prime` = 5, we can start at 5 * 5 because 5 * 3 was already counted
            // when `prime` was 3.
            // But here we do NOT want to do that, because we want to count the multiple for _each_
            // prime. (So we want to count 15 both when `prime` is 5 and when `prime` is 3).
            for multiple in ((prime * 2)..right).step_by(prime) {
                // Not a prime, since it's a multiple of `value`
                is_prime[multiple] = false;
                multiples_in_sequence[prime] += frequency[multiple];
            }
        }
    }
    // Now for each prime, we know how many multiples it has in our sequence.
    // The challenge now is how to answer the queries fast.

    let prefix_sum = {
        let mut res = Vec::with_capacity(7_000_000); // #primes in range.
        for &p in &all_primes {
            res.push(multiples_in_sequence[p]);
        }

        for i in 1..res.len() {
            res[i] += res[i - 1];
        }

        res
    };
    let get_sum = |l, r| {
        if l == 0 {
            prefix_sum[r]
        } else {
            prefix_sum[r] - prefix_sum[l - 1]
        }
    };

    let queries = input.read_size();
    for _ in 0..queries {
        let (l, r) = (input.read_size(), input.read_size());
        let l = all_primes.lower_bound(&l);
        let r = all_primes.upper_bound(&r) - 1;
        out_line!(get_sum(l, r));
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
