use itertools::Itertools;

pub fn permutations(size: usize) -> impl Iterator<Item = Vec<usize>> {
    let mut v: Vec<usize> = Vec::new();
    for i in 1..size + 1 {
        v.push(i);
    }
    return v.into_iter().permutations(size);
}

#[cfg(test)]
mod tests {
    #[test]
    fn generating_permutations() {
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(
            super::permutations(3).collect::<Vec<Vec<usize>>>(),
            expected
        );
    }
}
