use std::{collections::HashMap, hash};

use crate::solver;

pub struct DynamicSolver();

#[derive(Hash)]
struct SubPath {
    nodes: Vec<usize>,
    last_node: usize,
}

impl PartialEq for SubPath {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes && self.last_node == other.last_node
    }
}

impl Eq for SubPath {}

impl Clone for SubPath {
    fn clone(&self) -> Self {
        SubPath {
            nodes: self.nodes.clone(),
            last_node: self.last_node,
        }
    }
}

struct OrderedF32(f32);

impl OrderedF32 {
    fn key(&self) -> u32 {
        self.0.to_bits()
    }

    fn value(&self) -> f32 {
        self.0
    }
}

impl PartialEq for OrderedF32 {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for OrderedF32 {}

impl PartialOrd for OrderedF32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key().partial_cmp(&other.key())
    }
}

impl Ord for OrderedF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key().cmp(&other.key())
    }
}

impl hash::Hash for OrderedF32 {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state)
    }
}

impl Clone for OrderedF32 {
    fn clone(&self) -> Self {
        OrderedF32(self.0)
    }
}

impl Copy for OrderedF32 {}

#[derive(Hash)]
struct Direction {
    total_cost: OrderedF32,
    prev_node: usize,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost && self.prev_node == other.prev_node
    }
}

impl Eq for Direction {}

impl Clone for Direction {
    fn clone(&self) -> Self {
        Direction {
            total_cost: self.total_cost.clone(),
            prev_node: self.prev_node,
        }
    }
}

fn retrace_optimal_path(memo: &HashMap<SubPath, Direction>, n: usize) -> (Vec<usize>, f32) {
    let mut points_to_retrace = (1..n + 1).collect::<Vec<usize>>();
    let mut full_path_memo: HashMap<SubPath, Direction> = HashMap::new();
    for (k, v) in memo {
        if k.nodes == points_to_retrace {
            full_path_memo.insert(k.clone(), v.clone());
        }
    }

    let mut path_key = full_path_memo
        .clone()
        .keys()
        .min_by(|x, y| memo[x].total_cost.partial_cmp(&memo[y].total_cost).unwrap())
        .unwrap()
        .clone();

    let mut last_point = path_key.last_node;
    let mut optimal_cost = memo[&path_key].total_cost.value();
    let mut optimal_path = vec![last_point];

    points_to_retrace = points_to_retrace
        .iter()
        .filter(|x| **x != last_point)
        .map(|x| *x)
        .collect::<Vec<usize>>();

    while memo[&path_key].prev_node != 1 {
        let entry = memo[&path_key].clone();
        last_point = entry.prev_node;
        path_key = SubPath {
            nodes: points_to_retrace.clone(),
            last_node: last_point,
        };
        optimal_path.push(last_point);
        optimal_cost += entry.total_cost.value();
    }

    optimal_path.insert(0, 1);
    optimal_cost += memo[&path_key].total_cost.value();

    (optimal_path, optimal_cost)
}

impl solver::SolveTSP for DynamicSolver {
    fn solve(&self, matrix: &Vec<Vec<f32>>) -> solver::TSPSolution {
        let n = matrix.len();
        let all_nodes = (1..n + 1).collect::<Vec<usize>>();

        let mut memo: HashMap<SubPath, Direction> = HashMap::new();
        for i in 0..n {
            memo.insert(
                SubPath {
                    nodes: vec![i + 1],
                    last_node: i + 1,
                },
                Direction {
                    total_cost: OrderedF32(0.0),
                    prev_node: i + 1,
                },
            );
        }

        let mut queue: Vec<(Vec<usize>, usize)> = Vec::new();
        for i in 0..n {
            queue.push((vec![i + 1], i + 1));
        }

        while queue.len() > 0 {
            let (prev_visited, prev_last_point) = queue.pop().unwrap();
            let prev_path = SubPath {
                nodes: prev_visited.clone(),
                last_node: prev_last_point,
            };
            let prev_dist = memo[&prev_path].total_cost;
            let to_visit = all_nodes
                .iter()
                .filter(|x| !prev_visited.contains(x))
                .map(|x| *x)
                .collect::<Vec<usize>>();

            for new_last_point in to_visit {
                let mut new_visited = prev_visited.clone();
                new_visited.push(new_last_point);
                let new_dist = prev_dist.value() + matrix[prev_last_point - 1][new_last_point - 1];

                let key = SubPath {
                    nodes: new_visited.clone(),
                    last_node: new_last_point,
                };
                let direction = Direction {
                    total_cost: OrderedF32(new_dist),
                    prev_node: prev_last_point,
                };
                if !memo.contains_key(&key) {
                    memo.insert(key, direction);
                    queue.push((new_visited, new_last_point));
                } else {
                    let old_dist = memo[&key].total_cost;
                    if new_dist < old_dist.value() {
                        memo.insert(key, direction);
                    }
                }
            }
        }

        let (optimal_path, optimal_cost) = retrace_optimal_path(&memo, n);
        return solver::TSPSolution {
            path: optimal_path,
            cost: optimal_cost,
        };
    }
}
