fn solve(n: u32) -> u32 {
    (1..n).filter(|&x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main() {
    project_euler::run(solve, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(10);
        assert_eq!(result, 23);
    }
}
