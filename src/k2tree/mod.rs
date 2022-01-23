

use std::collections::VecDeque;
use crate::matrix::Matrix;


pub struct K2tree<T> where T:Clone{
	origin:Matrix<T>,
	k:usize,
	nodes:Vec<Option<T>>,
	leaf:Vec<T>
}

use core::fmt::Display;
impl <T> K2tree<T> where T:Display + Eq + Clone{
	pub fn new(matrix:Matrix<T>, k:usize) -> K2tree<T> {
		K2tree {
			origin:matrix,
			k,
			nodes:Vec::new(),
			leaf:Vec::new(),
		}
	}
	pub fn build(&mut self){
		
		let mut target = VecDeque::new();
		target.push_back(self.origin.submatrix(0..=self.origin.get_columns()-1, 0..=self.origin.get_rows()-1));
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
	pub fn get_nodes(&self)->&Vec<Option<T>>{
		&self.nodes
	}
	pub fn get_leaf(&self)->&Vec<T>{
		&self.leaf
	}
	pub fn get_k(&self)->usize{
		self.k
	}
}

mod tests {
	use crate::matrix::Matrix;
	use super::K2tree;
	#[test]
    fn test_works() {
		let size = 8;
        //let mut matrix = Matrix::from_iter(size,size,0..size*size);
		let mut matrix = Matrix::new(size,size);
		matrix.set(0, 2, 1);
		matrix.set(0, 3, 2);
		matrix.set(1, 2, 3);
		matrix.set(1, 3, 4);
		matrix.set(3,4,5);
		println!("{}", matrix);
		let mut k2tree = K2tree::new(matrix, 2);
		k2tree.build();
		println!("{:?}",k2tree.get_nodes());
		println!("{:?}",k2tree.get_leaf());

    }
}
