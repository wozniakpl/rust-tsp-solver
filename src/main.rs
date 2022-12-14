use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod brute_force;
mod dynamic;
mod solver;
mod utils;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_matrix(filename: &str) -> Vec<Vec<f32>> {
    let mut matrix: Vec<Vec<f32>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let mut row: Vec<f32> = Vec::new();
                for num in line.split_whitespace() {
                    row.push(num.parse::<f32>().unwrap());
                }
                matrix.push(row);
            }
        }
        return matrix;
    }
    panic!("Error reading file '{}'", filename);
}

fn solve(matrix: &Vec<Vec<f32>>, solver: &dyn solver::SolveTSP) -> solver::TSPSolution {
    return solver.solve(matrix);
}

fn main() {
    if std::env::args().len() <= 1 {
        println!("Usage: {} <filename>", std::env::args().nth(0).unwrap());
        return;
    }

    // TODO: parse args

    let input_filename = std::env::args().nth(1).expect("No input file given");
    let input_matrix = get_matrix(&input_filename);
    // TODO: different solver based on args
    let solution = solve(&input_matrix, &brute_force::BruteForceSolver {});
    println!("{:?} (cost {})", solution.path, solution.cost);

    // TODO: if 2nd arg provided, compare result with that file
}
// https://people.sc.fsu.edu/~jburkardt/datasets/tsp/tsp.html

#[cfg(test)]
mod tests {
    #[test]
    fn zero_length_path() {
        let matrix = vec![vec![0.0]]; // 1x1 dim
        let solution = super::solve(&matrix, &super::brute_force::BruteForceSolver {});
        assert_eq!(solution.path, vec![1]);
        assert_eq!(solution.cost, 0.0);
    }

    #[test]
    fn simple_path() {
        fn solve_with_solver(solver: &dyn super::solver::SolveTSP) {
            let matrix = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
            let solution = super::solve(&matrix, solver);
            assert_eq!(solution.path, vec![1, 2]);
            assert_eq!(solution.cost, 2.0);
        }
        solve_with_solver(&super::brute_force::BruteForceSolver {});
        solve_with_solver(&super::dynamic::DynamicSolver {});
    }

    #[test]
    fn more_complex_path() {
        let matrix = vec![
            vec![0.0, 3.0, 4.0, 2.0, 7.0],
            vec![3.0, 0.0, 4.0, 6.0, 3.0],
            vec![4.0, 4.0, 0.0, 5.0, 8.0],
            vec![2.0, 6.0, 5.0, 0.0, 6.0],
            vec![7.0, 3.0, 8.0, 6.0, 0.0],
        ];
        let solution = super::solve(&matrix, &super::brute_force::BruteForceSolver {});
        assert_eq!(solution.path, vec![1, 3, 2, 5, 4]);
        assert_eq!(solution.cost, 19.0);
    }
}
