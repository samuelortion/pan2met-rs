//! predict metabolic pathways

#![warn(missing_docs)]

/* std use */

/* crate use */
use anyhow::Context as _;
use clap::Parser as _;

/* project use */
use pan2met::cli;
use pan2met::error;


mod input;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File  must exist in the current path
    if let Ok(lines) = read_lines() {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[hotpath::main]
fn main() -> error::Result<()> {
    // Parse argument
    let arguments = cli::Arguments::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(arguments.quiet())
        .verbosity(arguments.verbosity())
        .timestamp(arguments.timestamp())
        .init()
        .context("stderrlog already create a logger")?;

    log::info!("Start a metabolic pathway prediction.");

    let input_path = arguments.input();
    
    
    log::info!("End a metabolic pathway prediction.");

    Ok(())
}
