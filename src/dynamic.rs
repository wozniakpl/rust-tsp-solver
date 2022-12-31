use std::collections::HashMap;

use crate::solver;

pub struct DynamicSolver();

fn retrace_optimal_path(
    memo: &HashMap<(Vec<usize>, usize), (f32, usize)>,
    n: usize,
) -> (Vec<usize>, f32) {
    let mut points_to_retrace = (1..n + 1).collect::<Vec<usize>>();
    let mut full_path_memo: HashMap<(Vec<usize>, usize), (f32, usize)> = HashMap::new();
    for (k, v) in memo {
        if k.0 == points_to_retrace {
            full_path_memo.insert(k.clone(), v.clone());
        }
    }

    let mut path_key = full_path_memo
        .keys()
        .min_by(|x, y| memo[x].0.partial_cmp(&memo[y].0).unwrap())
        .unwrap()
        .clone();

    let mut last_point = path_key.1;
    let mut optimal_cost = memo[&path_key].0;
    let mut optimal_path = vec![last_point];

    points_to_retrace = points_to_retrace
        .iter()
        .filter(|x| **x != last_point)
        .map(|x| *x)
        .collect::<Vec<usize>>();

    while memo[&path_key].1 != 1 {
        let entry = memo[&path_key];
        last_point = entry.1;
        path_key = (points_to_retrace.clone(), last_point);
        optimal_path.push(last_point);
        optimal_cost += entry.0;
    }

    optimal_path.insert(0, 1);
    optimal_cost += memo[&path_key].0;

    (optimal_path, optimal_cost)
}

impl solver::SolveTSP for DynamicSolver {
    fn solve(&self, matrix: &Vec<Vec<f32>>) -> solver::TSPSolution {
        let n = matrix.len();
        let all_nodes = (1..n + 1).collect::<Vec<usize>>();

        let mut memo: HashMap<(Vec<usize>, usize), (f32, usize)> = HashMap::new();
        for i in 0..n {
            memo.insert((vec![i + 1], i + 1), (0.0, i + 1));
        }

        let mut queue: Vec<(Vec<usize>, usize)> = Vec::new();
        for i in 0..n {
            queue.push((vec![i + 1], i + 1));
        }

        while queue.len() > 0 {
            let (prev_visited, prev_last_point) = queue.pop().unwrap();
            let prev_dist = memo[&(prev_visited.clone(), prev_last_point)].0;
            let to_visit = all_nodes
                .iter()
                .filter(|x| !prev_visited.contains(x))
                .map(|x| *x)
                .collect::<Vec<usize>>();

            for new_last_point in to_visit {
                let mut new_visited = prev_visited.clone();
                new_visited.push(new_last_point);
                let new_dist = prev_dist + matrix[prev_last_point - 1][new_last_point - 1];

                let key = (new_visited.clone(), new_last_point);
                if !memo.contains_key(&key) {
                    memo.insert(key, (new_dist, prev_last_point));
                    queue.push((new_visited, new_last_point));
                } else {
                    let (old_dist, _) = memo[&key];
                    if new_dist < old_dist {
                        memo.insert(key, (new_dist, prev_last_point));
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
