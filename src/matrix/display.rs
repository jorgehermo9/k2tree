
use super::Matrix;
use std::fmt;
use core::fmt::Display;

impl <T> Display for Matrix<T> where T:Display+Default{
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
