
use super::{Matrix,Submatrix};
use std::fmt;
use core::fmt::Display;

impl <T> Display for Matrix<T> where T:Display {
	fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result {
		let mut output = String::new();
		for i in 0..self.rows{
			for j in 0..self.columns-1{
				output.push_str(&format!("{:>3} ",self.get(i,j).unwrap()));
			}
			output.push_str(&format!("{:>3}\n",self.get(i,self.columns-1).unwrap()));
		}
		write!(f,"{}",output)
	}
}
impl <'a,T> Display for Submatrix<'a,T> where T:Display {
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