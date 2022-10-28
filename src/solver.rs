use itertools::Itertools;

fn cost(solution: &Vec<usize>, matrix: &Vec<Vec<f32>>) -> f32 {
    let mut cost = 0.0;
    for i in 0..solution.len() - 1 {
        cost += matrix[solution[i] - 1][solution[i + 1] - 1];
    }
    cost += matrix[solution[solution.len() - 1] - 1][solution[0] - 1];
    return cost;
}

fn permutations(size: usize) -> impl Iterator<Item = Vec<usize>> {
    let mut v: Vec<usize> = Vec::new();
    for i in 1..size + 1 {
        v.push(i);
    }
    return v.into_iter().permutations(size);
}

pub struct TSPSolution {
    pub path: Vec<usize>,
    pub cost: f32,
}

pub trait SolveTSP {
    fn solve(&self, matrix: &Vec<Vec<f32>>) -> TSPSolution;
}

pub struct BruteForceSolver();

impl SolveTSP for BruteForceSolver {

    fn solve(&self, matrix: &Vec<Vec<f32>>) -> TSPSolution {
        let mut min_cost = std::f32::MAX;
        let mut min_solution = vec![];

        for permutation in permutations(matrix.len()) {
            let cost = cost(&permutation, matrix);
            if cost < min_cost {
                min_cost = cost;
                min_solution = permutation.clone();
            }
        }

        return TSPSolution {
            path: min_solution,
            cost: min_cost,
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculating_cost() {
        // 1.0 + 1.0 = 2.0
        assert_eq!(super::cost(&vec![1, 2], &vec![vec![0.0, 1.0], vec![1.0, 0.0]]), 2.0);

        // 1.0 + 2.0 + 3.0 = 6.0
        assert_eq!(super::cost(&vec![1, 2, 3], &vec![vec![0.0, 1.0, 2.0], vec![1.0, 0.0, 3.0], vec![2.0, 3.0, 0.0]]), 6.0);
    }

    #[test]
    fn generating_permutations() {
        let expected = vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]];
        assert_eq!(super::permutations(3).collect::<Vec<Vec<usize>>>(), expected);
    }
}