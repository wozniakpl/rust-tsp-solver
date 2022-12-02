use crate::solver;

pub struct DynamicSolver();

impl solver::SolveTSP for DynamicSolver {
    fn solve(&self, matrix: &Vec<Vec<f32>>) -> solver::TSPSolution {
        return solver::TSPSolution {
            path: vec![],
            cost: 0.0,
        };
    }
}