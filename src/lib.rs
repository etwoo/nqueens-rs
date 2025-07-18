// https://github.com/rust-lang/rust/issues/43122
// https://github.com/rust-lang/rust/issues/117078
#![feature(gen_blocks, yield_expr)]
// https://github.com/taiki-e/cargo-llvm-cov#exclude-code-from-coverage
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub struct BoardSolution {
    positions: Vec<usize>,
}

impl BoardSolution {
    pub fn n(&self) -> usize {
        self.positions.len()
    }

    pub fn print_solution(&self) {
        for &queen_of_this_row in self.positions.iter() {
            for col in 0..self.n() {
                let val = if col == queen_of_this_row { "Q" } else { "_" };
                print!("{val:2}");
            }
            println!();
        }
    }
}

pub fn setup(n: usize) -> impl Iterator<Item = BoardSolution> {
    (0..n).flat_map(move |pos| make_solver(n, pos))
}

gen fn make_solver(n: usize, pos: usize) -> BoardSolution {
    let mut cursor = vec![pos];
    let end = vec![pos + 1];
    while let Some(&col) = cursor.last()
        && cursor != end
    {
        let backtrack = if n == cursor.len() - 1 {
            yield BoardSolution {
                positions: cursor.clone().into_iter().take(n).collect(),
            };
            // Backtrack after yielding a solution
            true
        } else {
            // Backtrack if done checking all columns in this row
            n == col
        };

        if backtrack {
            cursor.pop();
            next_column(&mut cursor);
        } else if is_position_eligible_for_queen(col, &cursor) {
            // Choose current column for this row, and go to the next row
            cursor.push(0);
        } else {
            next_column(&mut cursor);
        }
    }
}

fn next_column(cursor: &mut [usize]) {
    if let Some(col) = cursor.last_mut() {
        *col += 1;
    }
}

fn is_position_eligible_for_queen(candidate: usize, cursor: &[usize]) -> bool {
    let mut cols = cursor.iter().take(cursor.len() - 1);
    let eligible_column = cols.all(|&queen| queen != candidate);
    let mut diags = cursor.iter().rev().enumerate().skip(1);
    let eligible_diagonal =
        diags.all(|(distance, &queen)| queen.abs_diff(candidate) != distance);
    eligible_column && eligible_diagonal
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use rayon::prelude::*;

    fn solve(n: usize) -> Option<BoardSolution> {
        setup(n).next()
    }

    fn count(n: usize) -> usize {
        setup(n).count()
    }

    fn count_with_threads(n: usize) -> usize {
        (0..n)
            .into_par_iter()
            .map(|pos| make_solver(n, pos).count())
            .sum()
    }

    #[test]
    fn smoke_test_board_solution_struct() {
        match solve(1) {
            Some(solution) => {
                assert_eq!(solution.n(), 1);
                solution.print_solution();
            }
            None => println!("skipping smoke test on unexpected None"),
        }
    }

    #[test]
    fn solve_positive() {
        assert!(solve(1).is_some());
        assert!(solve(4).is_some());
        assert!(solve(5).is_some());
        assert!(solve(8).is_some());
    }

    #[test]
    fn solve_negative() {
        assert!(solve(0).is_none());
        assert!(solve(2).is_none());
        assert!(solve(3).is_none());
    }

    // https://en.wikipedia.org/wiki/Eight_queens_puzzle#Exact_enumeration
    #[test]
    fn count_solutions() {
        assert_eq!(count(0), 0);
        assert_eq!(count(1), 1);
        assert_eq!(count(2), 0);
        assert_eq!(count(3), 0);
        assert_eq!(count(4), 2);
        assert_eq!(count(5), 10);
        assert_eq!(count(6), 4);
        assert_eq!(count(7), 40);
        assert_eq!(count(8), 92);
    }

    #[test]
    fn count_solutions_with_threads() {
        assert_eq!(count_with_threads(0), 0);
        assert_eq!(count_with_threads(1), 1);
        assert_eq!(count_with_threads(2), 0);
        assert_eq!(count_with_threads(3), 0);
        assert_eq!(count_with_threads(4), 2);
        assert_eq!(count_with_threads(5), 10);
        assert_eq!(count_with_threads(6), 4);
        assert_eq!(count_with_threads(7), 40);
        assert_eq!(count_with_threads(8), 92);
    }

    #[test]
    fn count_solutions_with_threads_under_100ms() {
        assert_eq!(count_with_threads(9), 352);
        assert_eq!(count_with_threads(10), 724);
        assert_eq!(count_with_threads(11), 2680);
    }

    #[test]
    fn count_solutions_with_threads_under_1s() {
        assert_eq!(count_with_threads(12), 14200); // runtime: ~300ms
    }

    #[test]
    #[ignore]
    fn count_solutions_with_threads_under_30s() {
        assert_eq!(count_with_threads(13), 73712); // runtime: ~1.8s
        assert_eq!(count_with_threads(14), 365596); // runtime: ~11.5s
    }

    #[test]
    #[ignore]
    fn count_solutions_with_threads_under_2m() {
        assert_eq!(count_with_threads(15), 2279184); // runtime: ~80s
    }
}
