

use std::{collections::VecDeque, fmt::Debug};
use core::fmt::Display;

use crate::matrix::Matrix;
use crate::sequence::Sequence;
#[derive(Debug)]

pub struct K2tree<T> where T:Clone{
	rows:usize,
	columns:usize,
	virtual_rows:usize,
	virtual_cols:usize,
	k:usize,
	nodes:Sequence<Option<T>>,
	leaf:Vec<T>
}

fn next_pow(base:usize,n:usize)->usize{
	base.pow(f64::from(n as u32).log2().ceil() as u32)
}
impl <T> K2tree<T> where T:Display + Eq + Clone + Default{
	
	pub fn new(matrix:Matrix<T>, k:usize) -> K2tree<T> {
		let rows = matrix.get_rows();
		let columns = matrix.get_cols();
		let size = std::cmp::max(next_pow(k,rows),next_pow(k,columns));
		let mut tree = K2tree {
			rows,
			columns,
			virtual_rows:size,
			virtual_cols:size,
			k,
			nodes:Sequence::new(Vec::new(),None),
			leaf:Vec::new(),
		};
		tree.build(matrix.expand(size,size));
		return tree;
	}
	pub fn build(&mut self,matrix:Matrix<T>){
		
		let mut target = VecDeque::new();
		target.push_back(matrix.submatrix(0..=matrix.get_cols()-1, 0..=matrix.get_rows()-1));
		while target.len() > 0{
			let current = target.pop_front().unwrap();
			let mut ranges = Vec::new();
			let elem_c = current.get_cols()/self.k;
			let elem_r = current.get_rows()/self.k;
			
			for i in 0..self.k{
				for j in 0..self.k{
					ranges.push((i*elem_r..=(i+1)*elem_r-1,j*elem_c..=(j+1)*elem_c-1));
				}
			}
			for (x,y) in ranges{
				let submatrix = current.submatrix(y, x);
				if submatrix.elems() == 1{
					self.leaf.push(submatrix.get(0,0).unwrap().clone());
					continue;
				}
				let first = submatrix.get(0,0).unwrap();
				let all_eq = submatrix.iter().all(|item| item == first);
				if all_eq{
					self.nodes.push(Some(first.clone()));
				}else{
					self.nodes.push(None);
					target.push_back(submatrix);
				}
			}
		}
	}
	pub fn get(&self,i:usize,j:usize)-> Option<&T>{

		assert!(i<self.get_rows() && j<self.get_cols(),
		"position overflows k2tree");
		
		let mut l =1;
		let mut previous = 0;
		let mut virtual_x = j;
		let mut virtual_y = i;
		

		loop{
			
			//elems_c and elems_r will always be the same
			let elems_c = self.virtual_cols/self.k.pow(l);
			let elems_r = self.virtual_rows/self.k.pow(l);
			let x_node = virtual_x/elems_c;
			let y_node =virtual_y/elems_r;
			
			let pos = previous * self.k.pow(2) + y_node * self.k + x_node;
			//println!("previous: {},x_node:{},y_node:{},pos:{},elemns:{}",previous,x_node,y_node,pos,elems_c);
			
			if pos >= self.nodes.len(){
			//	println!("final pos: {}",pos - self.nodes.len());
				return self.leaf.get(pos-self.nodes.len())
			}
			match self.nodes.get(pos).unwrap(){
				None => {
					l+=1;
					previous = self.nodes.rank(pos).unwrap();
					virtual_x = virtual_x % elems_c;
					virtual_y = virtual_y % elems_r;
				},
				Some(n) => return Some(n)
			}
		}
		
	}
	pub fn get_nodes(&self)->&Sequence<Option<T>>{
		&self.nodes
	}
	pub fn get_leaf(&self)->&Vec<T>{
		&self.leaf
	}
	pub fn get_k(&self)->usize{
		self.k
	}
	pub fn get_rows(&self)->usize{
		self.rows
	}
	pub fn get_cols(&self) ->usize{
		self.columns
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;
	use super::K2tree;
	use rand::Rng;

	#[test]
	fn test_simple(){
		let size =4;
		let mut matrix = Matrix::from_iter(size,size,(0..size*size).
	map(|_| 0 ));
		matrix.set(0,1,1);
		println!("{}", matrix);

		let k2tree =K2tree::new(matrix,2);
		for i in 0..2{
			for j in 0..2{
				println!("{:?}", k2tree.get(i,j));
			}
		}
	}

	#[test]
    fn test_compression() {
		let size = 512;
		let mut rng = rand::thread_rng();
		let matrix:Matrix<usize> = Matrix::from_iter(size,size,
			(0..size*size).map(|_|rng.gen::<usize>() % 2));

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

	#[test]
	fn test_csv(){

		let mut reader = csv::Reader::from_path("/home/jorge/datasets/connect-4.data").unwrap();
		let mut features = Vec::new();

		for feature in reader.headers().unwrap(){
			features.push(String::from(feature));
		}
		let mut entities = Vec::new();

		for result in reader.records(){
			let record = result.unwrap();
			for value in record.into_iter(){
				entities.push(String::from(value));
			}
		}
		let n_features = features.len();
		let n_entities = entities.len()/n_features;
		let matrix = Matrix::from_iter(n_entities,n_features,entities.into_iter());
		let k2tree = K2tree::new(matrix, 2);

		let mut size_nodes = k2tree.get_nodes().get_data().len() * std::mem::size_of::<Option<String>>();
		size_nodes+=k2tree.get_nodes().get_target_index().len() * std::mem::size_of::<(usize,usize)>();
		let size_leaves = k2tree.get_leaf().len() * std::mem::size_of::<String>();
		let size_static = std::mem::size_of::<K2tree<String>>();
		let total_size = size_static+size_nodes+size_leaves;
		let raw_size = n_features*n_entities *std::mem::size_of::<String>();
		println!("K2tree size (bytes) {{static:{}, nodes: {}, leaves: {}}}: {}",size_static,size_nodes,size_leaves,total_size);
		println!("raw size (bytes): {}",raw_size);
		println!("compression rate: {}%", (1.0 - (total_size as f64/raw_size as f64)) * 100.0);
		
	}
}
