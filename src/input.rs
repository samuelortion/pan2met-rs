use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Read lines from a file
/// The output is wrapped in a Result to allow matching on errors.
/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Read rows of a file and put them into an HashSet
/// Returns a HashSet with each row of the file
pub fn read_set<P>(filename: P) -> io::Result<HashSet<String>>
where
    P: AsRef<Path>,
{
    let mut set: HashSet<String> = HashSet::new();
    let lines = read_lines(filename)?;
    for line in lines.map_while(Result::ok) {
        set.insert(line.clone());
    }
    Ok(set)
}
