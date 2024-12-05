use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    // Parse files
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut iter = line.split_whitespace();
            list_one.push(iter.next().unwrap().parse::<i32>().unwrap());
            list_two.push(iter.next().unwrap().parse::<i32>().unwrap());
        }
    }

    // Sort the lists
    list_one.sort();
    list_two.sort();

    let mut total: i32 = 0;
    for i in 0..list_one.len() {
        // Find the difference between the two lists
        let left = list_one[i];
        let right = list_two[i];
        let diff = left - right;

        // Add the absolute value to the total
        total += diff.abs();
    }

    println!("{}", total);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
