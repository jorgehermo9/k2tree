

use crate::matrix::Matrix;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn from_file(filename: &str,nodes:usize) -> Matrix<bool>{
	let mut matrix = Matrix::new(nodes,nodes);
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if !line.starts_with("#"){
					let mut splitted = line.split("\t");
					let from = splitted.next().unwrap();
					let to = splitted.next().unwrap();
					matrix.set(from.parse().unwrap(),to.parse().unwrap(),true);
				
				}
            }
        }
    }
	matrix
}
#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test(){
		from_file("/home/jorge/Downloads/firefox/web-Google.txt",10000);
	}
}