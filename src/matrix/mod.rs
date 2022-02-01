


use std::ops::RangeInclusive;

pub mod iter;
pub mod display;
pub mod submatrix;

//Shortcut for submatrix module
pub use submatrix::Submatrix;

pub struct Matrix<T>{
	inner:Vec<T>,
	rows:usize,
	columns:usize
}


impl <T> Matrix<T>{
	pub fn new(rows:usize,columns:usize) -> Matrix<T> where T:Default{
		Matrix::from_iter(rows,columns,(0..).map(|_| T::default()))
	}
	//if iterator has more elements than needed to fill the matrix, this method wont fail
	pub fn from_iter(rows:usize,columns:usize,input:impl IntoIterator<Item=T>) -> Self{
		assert!(rows>0 && columns>0);
		Matrix{
			inner:{
				let inner:Vec<T> = input.into_iter().take(rows*columns).collect();
				assert_eq!(rows*columns,inner.len(),"iterator cannot fill matrix");
				inner
			},
			rows,
			columns
		}
	}

	pub fn iter<'a>(&'a self) ->std::slice::Iter<'a,T>{
		self.into_iter()
	}
	pub fn iter_mut<'a>(&'a mut self) ->std::slice::IterMut<'a,T>{
		self.into_iter()
	}
	pub fn get(&self,i:usize,j:usize) ->Option<&T>{
		self.inner.get(i*self.columns+j)
	}
	pub fn get_mut(&mut self,i:usize,j:usize) ->&mut T{
		&mut self.inner[i*self.columns+j]
	}
	pub fn set(&mut self,i:usize,j:usize,value:T){
		self.inner[i*self.columns+j] = value;
	}
	pub fn get_columns(&self) -> usize{
		self.columns
	}
	pub fn get_rows(&self) -> usize{
		self.rows
	}
	pub fn get_inner(&self) -> &Vec<T>{
		&self.inner
	}
	pub fn get_inner_mut(&mut self)-> &mut Vec<T>{
		&mut self.inner
	}
	
	pub fn submatrix(&self,y:RangeInclusive<usize>,x:RangeInclusive<usize>)->Submatrix<T>{
		if x.is_empty(){panic!("x range must not be empty")}
		else if y.is_empty(){panic!("y range must not be empty")}

		if *x.end() > self.columns-1{panic!("x range overflows matrix")}
		else if *y.end() > self.rows-1{panic!("y range overflows matrix")}

		Submatrix::new(self,y,x)
	}
	pub fn expand(self,rows:usize,columns:usize)-> Self where T: Default + Clone{
		//TODO: Implement without cloning
		if rows <self.rows || columns <self.columns{
			panic!("Can't squeeze below current size");
		};
		if rows == self.rows && columns == self.columns{
			return self;
		}
		let mut new = Vec::new();
		for i in 0..rows{
			for j in 0..columns{
				if j>= self.columns || i>= self.rows{
					new.push(T::default());
				}else{
					new.push(self.get(i,j).unwrap().clone());
				}
			}
		}
		Matrix{
			inner:new,
			rows,
			columns
		}
	}

}


impl <T> Clone for Matrix<T> where T: Clone{
	fn clone(&self) -> Self{
		return Matrix{
			inner:self.inner.clone(),
			rows:self.rows,
			columns:self.columns
		}
	}
}


#[cfg(test)]
mod tests {
	use super::Matrix;
    #[test]
    fn new_test() {

		let expected = [[1,2],[3,4]];

		let mut matrix = Matrix::new(2,2);
		matrix.set(0,0,1);
		matrix.set(0,1,2);
		matrix.set(1,0,3);
		matrix.set(1,1,4);

		assert_eq!(matrix.get_columns(),2);
		assert_eq!(matrix.get_rows(),2);

		for i in 0..2{
			for j in 0..2{
				assert_eq!(expected[i][j],*matrix.get(i, j).unwrap());
			}
		}
    }

	#[test]
	fn from_vec_test(){
		let expected = [[1,2],[3,4]];
		let source = vec![1,2,3,4];
		let matrix = Matrix::from_iter(2,2,source);
		
		assert_eq!(matrix.get_columns(),2);
		assert_eq!(matrix.get_rows(),2);

		for i in 0..2{
			for j in 0..2{
				assert_eq!(expected[i][j],*matrix.get(i,j).unwrap())
			}
		}
	}
	#[test]
	fn to_iterator_test(){
		let expected = vec![1,2,3,4];
		let matrix = Matrix::from_iter(2,2,expected.clone());
		
		assert_eq!(matrix.get_columns(),2);
		assert_eq!(matrix.get_rows(),2);

		let flat_matrix:Vec<i32> = matrix.into_iter().collect();

		assert_eq!(flat_matrix,expected);
	}

	#[test]
	fn submatrix_test(){
		let expected = vec![1,2,3,4,5,6,7,8,9];
		let matrix = Matrix::from_iter(3,3,expected.clone());

		let submatrix = matrix.submatrix(0..=1,1..=2);
		println!("{}",submatrix);
	}

	#[test]
	fn submatrix_sum(){
		let expected = vec![1,2,3,4,5,6,7,8,9];
		let matrix = Matrix::from_iter(3,3,expected.clone());

		let submatrix = matrix.submatrix(0..=1,0..=1);

		let sum = submatrix.iter().fold(0, |acc,elem| acc + elem);

		assert_eq!(12,sum);
	}

	#[test]
	fn submatrix_sum_big(){
		let size = 1000;
		let matrix = Matrix::from_iter(size,size,0..size*size);

		let sum = matrix.into_iter().fold(0, |acc,elem|acc+elem);
		assert_eq!(sum,499999500000);
	}

	#[test]
	fn matrix_into_iter(){
		let size = 3;
		let matrix = Matrix::from_iter(size,size,0..size*size);

		for element in &matrix{
			println!("{}", element)
		}
		println!("{}",matrix)
	}
	
	
	
}