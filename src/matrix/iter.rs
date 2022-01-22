use super::Matrix;


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