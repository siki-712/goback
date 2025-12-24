mod reflog;

use std::env;
use std::process::{Command, exit};

struct Args {
    n: usize,
    print_only: bool,
    list: bool,
    list_count: usize,
}

fn print_help() {
    println!("goback - Go back to a previous git branch

USAGE:
    goback [N] [OPTIONS]

ARGS:
    N    Number of steps back (default: 1)

OPTIONS:
    -p, --print     Print the branch name only (don't switch)
    -l, --list [N]  List recent N branches (default: 10)
    -h, --help      Print this help message

EXAMPLES:
    goback           Go back to the previous branch
    goback 2         Go back 2 branches ago
    goback --print   Print the previous branch name
    goback 2 -p      Print the branch name 2 steps back
    goback -l        List recent 10 branches
    goback -l 5      List recent 5 branches");
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut n = 1;
    let mut print_only = false;
    let mut list = false;
    let mut list_count = 10;
    let mut expect_list_count = false;

    for arg in &args {
        if arg == "--help" || arg == "-h" {
            print_help();
            exit(0);
        } else if arg == "--list" || arg == "-l" {
            list = true;
            expect_list_count = true;
        } else if arg == "--print" || arg == "-p" {
            print_only = true;
        } else if let Ok(num) = arg.parse::<usize>() {
            if expect_list_count {
                list_count = num;
                expect_list_count = false;
            } else {
                n = num;
            }
        }
    }

    Args { n, print_only, list, list_count }
}

fn get_reflog() -> Result<String, String> {
    let output = Command::new("git")
        .args(["reflog"])
        .output()
        .map_err(|e| format!("failed to execute git reflog: {}", e))?;

    if !output.status.success() {
        return Err("failed to execute git reflog".to_string());
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("failed to parse output: {}", e))
}

fn checkout_branch(branch: &str) -> Result<(), String> {
    let status = Command::new("git")
        .args(["checkout", branch])
        .status()
        .map_err(|e| format!("failed to execute git checkout: {}", e))?;

    if !status.success() {
        return Err(format!("failed to switch to branch '{}'", branch));
    }

    Ok(())
}

fn print_list(history: &[String], count: usize) {
    if history.is_empty() {
        println!("No branch history found");
        return;
    }

    let display_count = count.min(history.len());
    for (i, branch) in history.iter().take(display_count).enumerate() {
        if i == 0 {
            println!("  {} : {} (current)", i, branch);
        } else {
            println!("  {} : {}", i, branch);
        }
    }
}

fn main() {
    let args = parse_args();

    let reflog = match get_reflog() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    };

    let history = reflog::extract_branch_history(&reflog);

    if args.list {
        print_list(&history, args.list_count);
        return;
    }

    if let Some(branch) = reflog::get_nth_previous_branch(&history, args.n) {
        if args.print_only {
            println!("{}", branch);
        } else {
            if let Err(e) = checkout_branch(branch) {
                eprintln!("error: {}", e);
                exit(1);
            }
        }
    } else {
        eprintln!("error: branch {} steps back not found", args.n);
        exit(1);
    }
}
