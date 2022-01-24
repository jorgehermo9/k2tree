
use super::Sequence;

impl <T> IntoIterator for Sequence<T>{
	type Item = T;
	type IntoIter= std::vec::IntoIter<T>;
	fn into_iter(self) -> Self::IntoIter{
		self.data.into_iter()
	}
}

impl <'a,T> IntoIterator for &'a Sequence<T>{
	type Item=&'a T;
	type IntoIter =std::slice::Iter<'a,T>;

	fn into_iter(self) -> Self::IntoIter{
		self.data.iter()
	}
}

impl <'a,T> IntoIterator for &'a mut Sequence<T>{
	type Item=&'a mut T;
	type IntoIter =std::slice::IterMut<'a,T>;

	fn into_iter(self) -> Self::IntoIter{
		self.data.iter_mut()
	}
}