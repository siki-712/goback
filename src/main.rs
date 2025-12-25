mod cli;
mod git;
mod reflog;
mod table;

use std::process::exit;

fn main() {
    let args = cli::parse_args();

    let reflog = match git::get_reflog() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    };

    let history = reflog::extract_branch_history(&reflog);

    if args.list {
        cli::print_list(&history, args.list_count);
        return;
    }

    if let Some(branch) = reflog::get_nth_previous_branch(&history, args.n) {
        if args.print_only {
            println!("{}", branch);
        } else if let Err(e) = git::checkout_branch(branch) {
            eprintln!("error: {}", e);
            exit(1);
        }
    } else {
        eprintln!("error: branch {} steps back not found", args.n);
        exit(1);
    }
}
