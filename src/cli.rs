use crate::color;
use crate::table::{Cell, Table};
use std::process::exit;

pub struct Args {
    pub n: usize,
    pub print_only: bool,
    pub list: bool,
    pub list_count: usize,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().skip(1).collect();
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

pub fn print_list(history: &[String], count: usize) {
    if history.is_empty() {
        println!("No branch history found");
        return;
    }

    let mut table = Table::new(vec!["#", "Branch", "Status"]);

    let display_count = count.min(history.len());
    for (i, branch) in history.iter().take(display_count).enumerate() {
        let num = Cell::colored(color::green_light(&i.to_string()), i.to_string().len());
        let branch_cell = Cell::new(branch);
        let status = if i == 0 {
            Cell::colored(color::green_light("current"), 7)
        } else {
            Cell::new("")
        };
        table.add_row(vec![num, branch_cell, status]);
    }

    print!("{}", table.render());
}
