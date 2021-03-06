use super::Matrix;
use std::ops::RangeInclusive;
use std::fmt;
use core::fmt::Display;

//TODO: All_eq method for submatrix in default region -> check if index in default
//region and return true
pub struct Submatrix<'a,T>{
	matrix:&'a Matrix<T>,
	x:RangeInclusive<usize>,
	y:RangeInclusive<usize>
}

pub struct SubmatrixIterator <'a,T>{
	submatrix:&'a Submatrix<'a,T>,
	current_i:usize,
	current_j:usize,
}
impl <'a,T> Iterator for SubmatrixIterator<'a,T> where T:Default{
	type Item=&'a T;

	fn next(&mut self) -> Option<&'a T>{
		
		if self.current_i >= self.submatrix.get_rows()
		 || self.current_j >= self.submatrix.get_cols(){
			
			return None;
		}
		match self.submatrix.get(self.current_i,self.current_j){
			None=>None,
			Some(n) => {
				if self.current_j + 1 == self.submatrix.get_cols(){
					self.current_j=0;
					self.current_i+=1;
				}else{
					self.current_j+=1;
				}
				Some(n)
			}
		}
	}
}
impl <'a,T> IntoIterator for &'a Submatrix<'a,T> where T:Default{
	type Item = &'a T;
	type IntoIter = SubmatrixIterator<'a,T>;
	fn into_iter(self) -> Self::IntoIter{
		SubmatrixIterator{
			submatrix:self,
			current_i:0,
			current_j: 0
		}
	}
}
impl <'a,T> Submatrix<'a,T> where T: Default{
	pub fn new(matrix:&'a Matrix<T>,y:RangeInclusive<usize>,x:RangeInclusive<usize>)->Self{
		Self{
			matrix,
			x,
			y
		}
	}
	pub fn get(&self,i:usize,j:usize) -> Option<&T>{
		if i > self.get_rows()-1 || j > self.get_cols()-1 {
			return None
		}
		
		let absolute_i = self.x.start() + i;
		let absolute_j = self.y.start()+j;
 		self.matrix.get(absolute_i,absolute_j)
	}

	//No funciona, iterador sobre toda la matriz
	pub fn iter(&self)->SubmatrixIterator<T>{
		self.into_iter()

	}
	pub fn get_rows(&self)->usize{
		self.y.end() - self.y.start()+1

	}
	pub fn get_cols(&self)->usize{
		self.x.end() - self.x.start()+1
	}
	//Ranges are offset from submatrix, not from original matrix
	pub fn submatrix(&self,y:RangeInclusive<usize>,x:RangeInclusive<usize>)->Submatrix<'a,T>{
		if x.is_empty(){panic!("x range must not be empty")}
		else if y.is_empty(){panic!("y range must not be empty")}

		if *x.end() > self.get_cols()-1{panic!("x range overflows matrix")}
		else if *y.end() > self.get_rows()-1{panic!("y range overflows matrix")}

		let x_elems = x.end() - x.start()+1;
		let x_absolute_start = self.x.start()+x.start();
		let absolute_x = x_absolute_start..=x_absolute_start+x_elems-1;

		let y_elems = y.end() - y.start()+1;
		let y_absolute_start = self.y.start()+y.start();
		let absolute_y  = y_absolute_start..=y_absolute_start+y_elems-1;
		
		Submatrix::new(self.matrix,absolute_y,absolute_x)
	}
	pub fn elems(&self)->usize{
		let x_elems = self.x.end() - self.x.start()+1;
		let y_elems = self.y.end() - self.y.start()+1;
		return x_elems * y_elems
	}
	pub fn all_eq(&self)->bool where T:Eq{
		//If all elements are default
		if *self.x.start()>=self.matrix.get_orig_cols() && *self.y.start()>=self.matrix.get_orig_rows(){
			return true;
		}
		let first = self.matrix.get(*self.x.start(),*self.y.start()).unwrap();
		let mut acc = true;
		for i in self.x.clone(){
			for j in self.y.clone(){
				acc &= *first == *self.matrix.get(i,j).unwrap();
			}
		}
		return acc;
	}
}

impl <'a,T> Display for Submatrix<'a,T> where T:Display+Default {
	fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
		let mut output = String::new();
		for i in 0..self.get_rows(){
			for j in 0..self.get_cols()-1{
				output.push_str(&format!("{:>3} ",self.get(i,j).unwrap()));
			}
			output.push_str(&format!("{:>3}\n",self.get(i,self.get_cols()-1).unwrap()));
		}
		write!(f,"{}",output)
	}
}

#[cfg(test)]
mod tests{
	use crate::matrix::Matrix;
	#[test]
	fn submatrix_size(){
		let size =1;
		let matrix = Matrix::from_iter(size,size,0..size*size);
		let submatrix = matrix.submatrix(0..=0,0..=0);
		assert_eq!(1,submatrix.elems());
	}

	#[test]
	fn submatrix_cols_rows(){
		let size =4;
		let matrix = Matrix::from_iter(size,size,0..size*size);
		let submatrix = matrix.submatrix(0..=1,0..=2);
		assert_eq!(submatrix.get_rows(),2);
		assert_eq!(submatrix.get_cols(),3);

		let subsubmatrix = submatrix.submatrix(0..=1,0..=0);
		assert_eq!(2,subsubmatrix.get_rows());
		assert_eq!(1,subsubmatrix.get_cols());
	}

	#[test]
	fn submatrix_iter(){
		let size=4;
		let matrix = Matrix::from_iter(size,size,0..size*size);
		let submatrix = matrix.submatrix(0..=1,0..=2);

		println!("Submatrix: ");
		println!("{}",submatrix);
		println!("Iterator:");
		for element in submatrix.iter() {
			print!("{} ",element);
		}
		println!("");
	}
}

