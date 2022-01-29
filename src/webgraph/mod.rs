

use crate::matrix::Matrix;
use crate::k2tree::K2tree;

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
                if !line.starts_with("#") && !line.starts_with("%"){
					let mut splitted = line.split("\t");
					let from = splitted.next().unwrap();
					let to = splitted.next().unwrap();
					matrix.set(from.parse::<usize>().unwrap()% 10000,to.parse::<usize>().unwrap()% 100000,true);
				
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
		let size = 10000;
		let matrix = from_file("/home/jorge/Downloads/firefox/web-Google.txt",size);
		//println!("{}",matrix);
		let k2tree = K2tree::new(matrix, 2);


		let mut size_nodes = k2tree.get_nodes().get_data().len() * std::mem::size_of::<Option<usize>>();
		size_nodes+=k2tree.get_nodes().get_target_index().len() * std::mem::size_of::<usize>();
		let size_leaves = k2tree.get_leaf().len() * std::mem::size_of::<usize>();
		let size_static = std::mem::size_of::<K2tree<usize>>();
		let total_size = size_static+size_nodes+size_leaves;
		let raw_size = size*size *std::mem::size_of::<usize>();
		println!("K2tree size (bytes) {{static:{}, nodes: {}, leaves: {}}}: {}",size_static,size_nodes,size_leaves,total_size);
		println!("raw size (bytes): {}",raw_size);
		println!("compression rate: {}%", (1.0 - (total_size as f64/raw_size as f64)) * 100.0);

	}
}