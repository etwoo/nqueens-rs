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
                print!("{:2}", val);
            }
            println!();
        }
    }
}

pub struct BoardState {
    n: usize,
    cursor: Vec<usize>,
    end: Vec<usize>,
}

impl Iterator for BoardState {
    type Item = BoardSolution;
    fn next(&mut self) -> Option<Self::Item> {
        self.choose()
    }
}

pub fn setup(n: usize) -> Vec<BoardState> {
    let mut v = Vec::new();
    for pos in 0..n {
        let b = BoardState {
            n,
            cursor: vec![pos],
            end: vec![pos + 1],
        };
        v.push(b);
    }
    v
}

impl BoardState {
    fn cursor_at_row(&self) -> usize {
        assert!(!self.cursor.is_empty());
        self.cursor.len() - 1
    }

    fn cursor_at_col(&self) -> usize {
        assert!(!self.cursor.is_empty());
        *self.cursor.last().unwrap()
    }

    fn is_cursor_eligible_for_queen(&self) -> bool {
        assert!(!self.cursor.is_empty());
        let cur = self.cursor_at_col();
        let n_to_take = self.cursor.len() - 1;
        if self.cursor.iter().take(n_to_take).any(|&x| x == cur) {
            // Existing queen already occupies this column
            return false;
        }
        for (distance, &queen) in self.cursor.iter().rev().enumerate() {
            if distance == 0 {
                // Don't consider `cur` as conflicting with itself
                assert_eq!(queen, cur);
                continue;
            }
            // Check if existing queen hits this position on a diagonal
            if queen.abs_diff(cur) == distance {
                return false;
            }
        }
        true
    }

    fn move_cursor_to_next_column(&mut self) {
        assert!(!self.cursor.is_empty());
        // Caller must check if we've exceeded `self.n`!
        *self.cursor.last_mut().unwrap() += 1;
    }

    fn move_cursor_to_next_row_first_column(&mut self) {
        self.cursor.push(0);
    }

    fn prepare_cursor_for_next(&mut self) {
        assert!(!self.cursor.is_empty());
        self.cursor.pop();
        self.move_cursor_to_next_column();
    }

    fn choose(&mut self) -> Option<BoardSolution> {
        loop {
            if self.cursor == self.end {
                return None;
            }

            if self.cursor_at_col() == self.n {
                self.prepare_cursor_for_next();
                continue;
            }

            if self.cursor_at_row() == self.n {
                let v = self.cursor.to_vec(); // deepcopy before cursor movement
                self.prepare_cursor_for_next();
                return Some(BoardSolution {
                    positions: v.into_iter().take(self.cursor.len()).collect(),
                });
            }

            if self.is_cursor_eligible_for_queen() {
                self.move_cursor_to_next_row_first_column();
                continue;
            }

            self.move_cursor_to_next_column();
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use rayon::prelude::*;

    fn solve(n: usize) -> Option<BoardSolution> {
        setup(n).into_iter().flatten().next()
    }

    fn count(n: usize) -> usize {
        setup(n).into_iter().map(|x| x.count()).sum()
    }

    fn count_with_threads(n: usize) -> usize {
        setup(n).into_par_iter().map(|x| x.count()).sum()
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
