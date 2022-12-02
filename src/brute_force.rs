use crate::solver;
use crate::utils;

pub struct BruteForceSolver();

impl solver::SolveTSP for BruteForceSolver {
    fn solve(&self, matrix: &Vec<Vec<f32>>) -> solver::TSPSolution {
        let mut min_cost = std::f32::MAX;
        let mut min_solution = vec![];

        for permutation in utils::permutations(matrix.len()) {
            let cost = solver::get_cost(&permutation, matrix);
            if cost < min_cost {
                min_cost = cost;
                min_solution = permutation.clone();
            }
        }

        return solver::TSPSolution {
            path: min_solution,
            cost: min_cost,
        };
    }
}
