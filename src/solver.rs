pub struct TSPSolution {
    pub path: Vec<usize>,
    pub cost: f32,
}

pub fn get_cost(solution: &Vec<usize>, matrix: &Vec<Vec<f32>>) -> f32 {
    let mut cost = 0.0;
    for i in 0..solution.len() - 1 {
        cost += matrix[solution[i] - 1][solution[i + 1] - 1];
    }
    cost += matrix[solution[solution.len() - 1] - 1][solution[0] - 1];
    return cost;
}

pub trait SolveTSP {
    fn solve(&self, matrix: &Vec<Vec<f32>>) -> TSPSolution;
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculating_cost() {
        // 1.0 + 1.0 = 2.0
        assert_eq!(super::get_cost(&vec![1, 2], &vec![vec![0.0, 1.0], vec![1.0, 0.0]]), 2.0);

        // 1.0 + 2.0 + 3.0 = 6.0
        assert_eq!(super::get_cost(&vec![1, 2, 3], &vec![vec![0.0, 1.0, 2.0], vec![1.0, 0.0, 3.0], vec![2.0, 3.0, 0.0]]), 6.0);
    }
}