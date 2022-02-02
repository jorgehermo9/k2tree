use super::Matrix;

pub struct MatrixIterator <'a,T>{
	matrix:&'a Matrix<T>,
	current_i:usize,
	current_j:usize,
}
/*
pub struct MatrixIteratorOwned<T>{
	matrix:Matrix<T>,
	current_i:usize,
	current_j:usize,
}
*/

impl <'a,T> Iterator for MatrixIterator<'a,T> where T:Default{
	type Item=&'a T;

	fn next(&mut self) -> Option<&'a T>{
		
		if self.current_i >= self.matrix.get_rows()
		 || self.current_j >= self.matrix.get_cols(){
			
			return None;
		}
		match self.matrix.get(self.current_i,self.current_j){
			None=>None,
			Some(n) => {
				if self.current_j + 1 == self.matrix.get_cols(){
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

/* 
impl <T> Iterator for MatrixIteratorOwned<T> where T:Default{
	type Item=T;

	fn next(&mut self) -> Option<T>{
		
		if self.current_i >= self.matrix.get_virtual_rows()
		 || self.current_j >= self.matrix.get_virtual_cols(){
			
			return None;
		}
		match self.matrix.get_or_default(self.current_i,self.current_j){
			None=>None,
			Some(n) => {
				if self.current_j + 1 == self.matrix.get_virtual_cols(){
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
*/
impl <'a,T> IntoIterator for &'a Matrix<T> where T:Default{
	type Item = &'a T;
	type IntoIter = MatrixIterator<'a,T>;
	fn into_iter(self) -> Self::IntoIter{
		MatrixIterator{
			matrix:self,
			current_i:0,
			current_j: 0
		}
	}
}

/* 
impl <T> IntoIterator for Matrix<T>{
	type Item = T;
	type IntoIter= std::vec::IntoIter<T>;
	fn into_iter(self) -> Self::IntoIter{
		self.inner.into_iter()
	}
}

impl <'a,T> IntoIterator for &'a Matrix<T>{
	type Item=&'a T;
	type IntoIter =std::slice::Iter<'a,T>;

	fn into_iter(self) -> Self::IntoIter{
		self.inner.iter()
	}
}

impl <'a,T> IntoIterator for &'a mut Matrix<T>{
	type Item=&'a mut T;
	type IntoIter =std::slice::IterMut<'a,T>;

	fn into_iter(self) -> Self::IntoIter{
		self.inner.iter_mut()
	}
}
*/