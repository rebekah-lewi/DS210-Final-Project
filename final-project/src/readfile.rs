use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_file(path: &str) -> io::Result<Vec<(usize, usize)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();


    // Reading the remaining lines to get the edges
    let edges: Vec<(usize, usize)> = lines
        .filter_map(|line| line.ok()) 
        .map(|line| {
            let parts = line.split(",").collect::<Vec<_>>();
            let src = parts[0].parse::<usize>();
            let dst = parts[1].parse::<usize>();
            match (src, dst) {
                (Ok(src), Ok(dst)) => Some((src, dst)),
                _ => None,
            }
        })
        .filter_map(|x| x) // Removing None values
        .collect();

    Ok(edges)
}