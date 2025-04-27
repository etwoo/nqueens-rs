use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing N argument");
        return ExitCode::FAILURE;
    }

    let n = match args[1].parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("cannot parse N value '{}': {}", args[1], e);
            return ExitCode::FAILURE;
        }
    };

    let mut solution_count = 0;
    for solution in nqueens::setup(n).into_iter().flatten() {
        solution_count += 1;
        println!("Got solution {} for n={}", solution_count, solution.n());
        solution.print_solution();
    }

    if solution_count == 0 {
        println!("No solution for n={}", n);
    } else {
        println!("Found {} solutions for n={}", solution_count, n);
    }

    ExitCode::SUCCESS
}
