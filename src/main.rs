use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
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
        return matrix
    }
    panic!("Error reading file {}", filename);
}

fn solve(matrix: &Vec<Vec<f32>>) -> Vec<usize> {
    let mut solution = vec![1];
    let mut visited_nodes = vec![1];

    // TODO: naive solution - choose always the next node with the lowest cost
    while visited_nodes.len() < matrix.len() {
        let mut min_cost = std::f32::MAX;
        let mut min_index = 0;
        for i in 0..matrix.len() {
            if !visited_nodes.contains(&(i+1)) {
                let cost = matrix[solution.last().unwrap() - 1][i];
                if cost < min_cost {
                    min_cost = cost;
                    min_index = i;
                }
            }
        }
        solution.push(min_index + 1);
        visited_nodes.push(min_index + 1);
    }

    return solution;
}

fn cost(solution: &Vec<usize>, matrix: &Vec<Vec<f32>>) -> f32 {
    let mut cost = 0.0;
    for i in 0..solution.len() - 1 {
        cost += matrix[solution[i] - 1][solution[i + 1] - 1];
    }
    cost += matrix[solution[solution.len() - 1] - 1][solution[0] - 1];
    return cost;
}

fn main() {
    let input_filename = std::env::args().nth(1).expect("no input file given");
    let input_matrix = get_matrix(&input_filename);
    let solution = solve(&input_matrix);
    println!("{:?} (cost {})", solution, cost(&solution, &input_matrix));

    // TODO: if 2nd arg provided, compare result with that file
}

// https://people.sc.fsu.edu/~jburkardt/datasets/tsp/tsp.html
#[cfg(test)]
mod tests {
    #[test]
    fn simple_path() {
        let matrix = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let solution = super::solve(&matrix);
        assert_eq!(solution, vec![1, 2]);
        assert_eq!(super::cost(&solution, &matrix), 2.0);
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
        let solution = super::solve(&matrix);
        assert_eq!(solution, vec![1, 3, 2, 5, 4]);
        assert_eq!(super::cost(&solution, &matrix), 19.0);
    }
}