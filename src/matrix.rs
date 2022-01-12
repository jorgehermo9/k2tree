pub mod matrix{
	use std::ops::RangeInclusive;
	use core::fmt::Display;
	pub struct Matrix<E> where E:Clone{
		inner:Vec<Vec<E>>,
		rows:usize,
		columns:usize
	}
	
	
	impl <E> Matrix<E> where E:Clone{
		pub fn new(rows:usize,columns:usize,default:E) -> Self{
			let mut inner= Vec::new();
			for _ in 0..rows{
				let row = vec![default.clone();columns];
				inner.push(row);
			}
			return Matrix{
				inner,
				rows,
				columns
			}
		}
		pub fn from_iterator(rows:usize,columns:usize,input:impl Iterator<Item=E>)-> Self{
			
			let mut iterator = input.into_iter();
			let mut inner= Vec::new();

			for _ in 0..rows{
				let mut row = Vec::new();
				for _ in 0..columns{
					match iterator.next(){
						Some(element) => row.push(element),
						None => panic!("Iterator has less elements than rows * columns")
					}
				}
				inner.push(row);
			}
			//Test if iterator is empty after 
			if let Some(_) = iterator.next(){
				panic!("Iterator has more elements than rows * columns")
			}
			Matrix{
				inner,
				rows,
				columns
			}
		}
		pub fn to_iterator(&self)->impl Iterator<Item=&E>{
			let mut inner_flat = Vec::new();
			for i in 0..self.rows{
				let row = &self.inner[i];
				for element in row{
					inner_flat.push(element);
				}
			}
			inner_flat.into_iter()
		}
		pub fn get(&self,i:usize,j:usize) ->&E{
			&self.inner[i][j]
		}
		pub fn set(&mut self,i:usize,j:usize,value:E){
			self.inner[i][j] = value;
		}
		pub fn get_columns(&self) -> usize{
			self.columns
		}
		pub fn get_rows(&self) -> usize{
			self.rows
		}
		pub fn get_inner(&self) -> &Vec<Vec<E>>{
			&self.inner
		}
		pub fn get_inner_mut(&mut self)-> &mut Vec<Vec<E>>{
			&mut self.inner
		}
		pub fn submatrix(&self,x:RangeInclusive<usize>,y:RangeInclusive<usize>)->Matrix<&E>{
			if x.is_empty(){panic!("x range must not be empty")}
			else if y.is_empty(){panic!("y range must not be empty")}

			if *x.end() > self.columns-1{panic!("x range overflows matrix")}
			else if *y.end() > self.rows-1{panic!("y range overflows matrix")}

			let mut inner = Vec::new();

			for i in y.clone(){
				let mut row = Vec::new();
				for j in x.clone(){
					row.push(&self.inner[i][j])
				}
				inner.push(row)
			}
			Matrix{
				inner,
				rows:x.count(),
				columns:y.count()
			}
		}

	}
	
	impl <E> Matrix<E> where E:Clone+Display{
		//Implement Display trait instead of print
		pub fn print(&self){
			for i in 0..self.rows{
				for j in 0..self.columns-1{
					print!("{} ",self.inner[i][j]);
				}
				print!("{}\n",self.inner[i][self.columns-1]);
			}
		}
	}
	
	impl <E> Clone for Matrix<E> where E:Clone{
		fn clone(&self) -> Self{
			return Matrix{
				inner:self.inner.clone(),
				rows:self.rows,
				columns:self.columns
			}
		}
	}

}

#[cfg(test)]
mod tests {
	use super::matrix::Matrix;
    #[test]
    fn new_test() {

		let expected = [[1,2],[3,4]];

		let mut matrix = Matrix::new(2,2,1);
		matrix.set(0,0,1);
		matrix.set(0,1,2);
		matrix.set(1,0,3);
		matrix.set(1,1,4);

		assert_eq!(matrix.get_columns(),2);
		assert_eq!(matrix.get_rows(),2);

		for i in 0..2{
			for j in 0..2{
				assert_eq!(expected[i][j],*matrix.get(i, j));
			}
		}
    }

	#[test]
	fn from_vec_test(){
		let expected = [[1,2],[3,4]];
		let source = vec![1,2,3,4];
		let matrix = Matrix::from_iterator(2,2,source.into_iter());
		
		assert_eq!(matrix.get_columns(),2);
		assert_eq!(matrix.get_rows(),2);

		for i in 0..2{
			for j in 0..2{
				assert_eq!(expected[i][j],*matrix.get(i,j))
			}
		}
	}
	#[test]
	fn to_iterator_test(){
		let expected = vec![1,2,3,4];
		let matrix = Matrix::from_iterator(2,2,expected.clone().into_iter());
		
		assert_eq!(matrix.get_columns(),2);
		assert_eq!(matrix.get_rows(),2);

		let flat_matrix:Vec<&i32> = matrix.to_iterator().collect();

		assert_eq!(flat_matrix,expected.iter().collect::<Vec<&i32>>());
	}

	#[test]
	fn submatrix_test(){
		let expected = vec![1,2,3,4,5,6,7,8,9];
		let matrix = Matrix::from_iterator(3,3,expected.clone().into_iter());
		matrix.print();

		let submatrix = matrix.submatrix(0..=1,1..=2);
		println!("");
		submatrix.print()
	}
	
}