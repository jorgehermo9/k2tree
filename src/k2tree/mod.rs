

use std::{collections::VecDeque, fmt::Debug};
use core::fmt::Display;

use crate::matrix::Matrix;
use crate::sequence::Sequence;
#[derive(Debug)]

pub struct K2tree<T> where T:Clone{
	rows:usize,
	columns:usize,
	k:usize,
	nodes:Sequence<Option<T>>,
	leaf:Vec<T>
}

impl <T> K2tree<T> where T:Display + Eq + Clone{
	pub fn new(matrix:Matrix<T>, k:usize) -> K2tree<T> {
		let mut tree = K2tree {
			rows: matrix.get_rows(),
			columns: matrix.get_columns(),
			k,
			nodes:Sequence::new(Vec::new(),None),
			leaf:Vec::new(),
		};
		tree.build(matrix);
		return tree;
	}
	pub fn build(&mut self,matrix:Matrix<T>){
		
		let mut target = VecDeque::new();
		target.push_back(matrix.submatrix(0..=matrix.get_columns()-1, 0..=matrix.get_rows()-1));
		while target.len() > 0{
			let current = target.pop_front().unwrap();
			let mut ranges = Vec::new();
			let elem_c = current.get_columns()/self.k;
			let elem_r = current.get_rows()/self.k;
	
			for i in 0..self.k{
				for j in 0..self.k{
					ranges.push((j*elem_c..=(j+1)*elem_c-1,i*elem_r..=(i+1)*elem_r-1));
				}
			}
			for (x,y) in ranges{
				let submatrix = current.submatrix(x, y);
				if submatrix.get_inner().len() == 1{
					self.leaf.push(submatrix.get(0,0).clone());
					continue;
				}
				let first = submatrix.get(0,0);
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
	pub fn get(&self,i:usize,j:usize)-> &T{

		assert!(i<self.get_rows() && j<self.get_columns(),
		"position overflows matrix");
		
		let mut l =1;
		let mut previous = 0;
		let mut virtual_x = j;
		let mut virtual_y = i;
		

		loop{
			
			let elems_c = self.get_columns()/self.k.pow(l);
			let elems_r = self.get_rows()/self.k.pow(l);
			let x_node = virtual_x/elems_c;
			let y_node =virtual_y/elems_r;
			
			let pos = previous * self.k.pow(2) + y_node * self.k + x_node;
			println!("previous: {},x_node:{},y_node:{},pos:{}",previous,x_node,y_node,pos);
			
			if pos >= self.nodes.len(){
				return &self.leaf[pos-self.nodes.len()]
			}
			match self.nodes.get(pos).unwrap(){
				None => {
					l+=1;
					previous = *self.nodes.rank(pos).unwrap();
					virtual_x = virtual_x % elems_c;
					virtual_y = virtual_y % elems_c;
					continue
				},
				Some(n) => return n
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
	pub fn get_columns(&self) ->usize{
		self.columns
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;
	use super::K2tree;
	#[test]
    fn test_works() {
		let size = 32;
        //let mut matrix = Matrix::from_iter(size,size,0..size*size);
		let mut matrix:Matrix<usize> = Matrix::from_iter(size,size,
			(0..size*size).map(|item| if item%8 ==0 || item%8 == 1 {1}else{0}));

		//let mut matrix:Matrix<usize> = Matrix::new(size,size);
		// matrix.set(0, 2, 1);
		// matrix.set(0, 3, 2);
		// matrix.set(1, 2, 3);
		// matrix.set(1, 3, 4);
		// matrix.set(3,4,5);
		// matrix.set(7,2,6);
		// matrix.set(7,7,2);
		println!("{}", matrix);
		let k2tree = K2tree::new(matrix, 2);
		println!("{:?}",k2tree.get_nodes());
		//println!("{:?}",k2tree.get_leaf());
		println!("{}",k2tree.get(1, 2));

		let mut size_nodes = k2tree.get_nodes().len() * std::mem::size_of::<Option<i32>>();
		size_nodes+=k2tree.get_nodes().get_target_index().len() * std::mem::size_of::<usize>();
		let size_leaves = k2tree.get_leaf().len() * std::mem::size_of::<i32>();
		let size_static = std::mem::size_of_val(&k2tree);
		let total_size = size_static+size_nodes+size_leaves;
		let raw_size = size*size *std::mem::size_of::<i32>();
		println!("dynamic size (bytes) {{static:{}, nodes: {}, leaves: {}}}: {}",size_static,size_nodes,size_leaves,total_size);
		println!("raw size (bytes): {}",raw_size);
		println!("compression rate: {}%", (1.0 - (total_size as f64/raw_size as f64)) * 100.0);
	}

}
