//{"name":"C. Civilization","group":"Codeforces - Codeforces Round 260 (Div. 1)","url":"https://codeforces.com/problemset/problem/455/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6 0 6\n2 1 2\n2 3 4\n2 5 6\n2 3 2\n2 5 3\n1 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCivilization"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction, RecursiveFunction2};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m, q) = (input.read_size(), input.read_size(), input.read_size());
    let mut graph = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b) = (input.read_size(), input.read_size());
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n + 1];
    let mut component_id = vec![0; n + 1];

    let mut current_component = 1;
    for v in 1..=n {
        if !visited[v] {
            let mut find_connected_components = RecursiveFunction2::new(|f, current, component| {
                if visited[current] {
                    return;
                }
                visited[current] = true;
                component_id[current] = component;
                for &neighbor in &graph[current] {
                    f.call(neighbor, component);
                }
            });
            find_connected_components.call(v, current_component);
            current_component += 1;
        };
    }

    let mut id_to_vertices = vec![vec![]; current_component + 1];
    for v in 1..=n {
        id_to_vertices[component_id[v]].push(v);
    }

    let mut dist = vec![i64::MIN; n + 1];
    let mut dsu = DSU::new(n + 1);
    for vertices in id_to_vertices {
        if vertices.is_empty() {
            continue;
        }
        let first_vertex = vertices[0];
        let mut find_distances = RecursiveFunction2::new(|f, current, d| {
            if dist[current] != i64::MIN {
                return;
            }
            dist[current] = d;

            for &neighbor in &graph[current] {
                f.call(neighbor, d + 1);
            }
        });
        find_distances.call(first_vertex, 0);

        let endpoint = vertices
            .iter()
            .max_by(|&&x, &&y| dist[x].cmp(&dist[y]))
            .unwrap();
        for v in &vertices {
            dist[*v] = i64::MIN;
        }
        let mut find_distances = RecursiveFunction2::new(|f, current, d| {
            if dist[current] != i64::MIN {
                return;
            }
            dist[current] = d;

            for &neighbor in &graph[current] {
                f.call(neighbor, d + 1);
            }
        });
        find_distances.call(*endpoint, 0);

        let diameter = vertices.iter().map(|x| dist[*x]).max().unwrap();
        for v in vertices {
            dsu.join(first_vertex, v);
        }
        dsu.set_longest_path(first_vertex, diameter as usize);
    }

    for _ in 0..q {
        let t = input.read_size();
        if t == 1 {
            let x = input.read_size();
            out_line!(dsu.get_longest_path(x));
        } else {
            let x = input.read_size();
            let y = input.read_size();
            dsu.join(x, y);
        }
    }
}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::legacy_fill::LegacyFill;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Bounds;
use std::cell::Cell;
use std::cmp::max;

#[derive(Clone)]
pub struct DSU {
    id: Vec<Cell<u32>>,
    size: Vec<u32>,
    count: usize,
    longest_path: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            id: (0..n).map(|i| Cell::new(i as u32)).collect_vec(),
            size: vec![1; n],
            count: n,
            longest_path: vec![0; n],
        }
    }

    pub fn set_longest_path(&mut self, i: usize, v: usize) {
        let repr = self.get(i);
        self.longest_path[repr] = v;
    }

    pub fn get_longest_path(&mut self, i: usize) -> usize {
        self.longest_path[self.get(i)]
    }

    pub fn size(&self, i: usize) -> usize {
        self.size[self.get(i)] as usize
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.id.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.id.iter().enumerate().filter_map(|(i, id)| {
            if (i as u32) == id.get() {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn set_count(&self) -> usize {
        self.count
    }

    pub fn join(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.get(a);
        b = self.get(b);
        if a == b {
            false
        } else {
            self.size[a] += self.size[b];
            self.id[b].replace(a as u32);
            let contribution_from_a = (self.longest_path[a] + 1) / 2;
            let contribution_from_b = (self.longest_path[b] + 1) / 2;
            let before = max(self.longest_path[a], self.longest_path[b]);
            self.longest_path[a] = max(before, contribution_from_a + contribution_from_b + 1);
            self.count -= 1;
            true
        }
    }

    pub fn get(&self, i: usize) -> usize {
        if self.id[i].get() != i as u32 {
            let res = self.get(self.id[i].get() as usize);
            self.id[i].replace(res as u32);
        }
        self.id[i].get() as usize
    }

    pub fn clear(&mut self) {
        self.count = self.id.len();
        self.size.legacy_fill(1);
        self.id.iter().enumerate().for_each(|(i, id)| {
            id.replace(i as u32);
        });
    }

    pub fn parts(&self) -> Vec<Vec<usize>> {
        let roots = self.iter().collect_vec();
        let mut res = vec![Vec::new(); roots.len()];
        for i in 0..self.id.len() {
            res[roots.as_slice().bin_search(&self.get(i)).unwrap()].push(i);
        }
        res
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
