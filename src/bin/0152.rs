fn search(terms: &Vec<f32>, suffix_sum: &Vec<f32>, i: usize, current: f32) -> u32 {
    // If we're done searching through all of the terms, stop.
    if i == terms.len() {
        return 0;
    }

    // If we're already greater than our goal, stop.
    if current > 0.5 {
        return 0;
    }

    // If our optimistic sum is less than our goal, stop.
    if current + suffix_sum[i] < 0.5 {
        return 0;
    }

    // If we are close enough to our goal, stop.
    if (current - 0.5).abs() < 1e-4 {
        return 1;
    }

    search(terms, suffix_sum, i + 1, current + terms[i]) + search(terms, suffix_sum, i + 1, current)
}

fn solve(n: u32) -> u32 {
    let _sq_terms: Vec<u32> = (2..=n).map(|x| x * x).collect();

    let terms: Vec<f32> = (2..=n).map(|x| 1.0 / ((x * x) as f32)).collect();

    let n_terms = terms.len();

    let mut suffix_sum = vec![0.0; n_terms];
    suffix_sum[n_terms - 1] = terms[n_terms - 1];
    for i in (0..n_terms - 1).rev() {
        suffix_sum[i] = suffix_sum[i + 1] + terms[i];
    }

    search(&terms, &suffix_sum, 0usize, 0f32)
}

fn main() {
    utils::run(solve, 20);
    // println!("{}", u128::MAX);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_solve() {
//         let result = solve(45);
//         assert_eq!(result, 3);
//     }
// }
