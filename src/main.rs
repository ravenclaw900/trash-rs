#![warn(clippy::pedantic)]

use getopts::Options;
use std::env;
use std::fs;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

fn print_usage(opts: &Options) {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for i in [
        "add",
        "move file to trash",
        "init",
        "init trash",
        "list",
        "list files in trash",
        "restore",
        "restore file from trash",
        "delete",
        "permanently delete a file from the trash",
        "empty",
        "empty all items from trash",
    ] {
        grid.add(Cell::from(i))
    }

    let usage = format!(
        "{} <command>
Available subcommands:
{}",
        opts.short_usage("trash"),
        grid.fit_into_columns(2)
    );
    print!("{}", opts.usage(&usage));
}

fn opt_setup(opts: &mut Options) -> Option<String> {
    let args: Vec<String> = env::args().skip(1).collect();

    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f)
        }
    };
    if matches.free.is_empty() || matches.opt_present("h") {
        print_usage(&opts);
        None
    } else {
        Some(matches.free[0].clone())
    }
}

fn main() {
    let mut opts = Options::new();
    let arg = match opt_setup(&mut opts) {
        Some(v) => v,
        None => return,
    };
    match arg.as_str() {
        "init" => {
            if fs::create_dir("TODO").is_err() {
                eprintln!("Couldn't create trash directory at TODO")
            }
        }
        _ => print_usage(&opts),
    }
}
