pub mod matrix{
	
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
						None => panic!("Iterator's length must match rows * columns")
					}
				}
				inner.push(row);
			}
			Matrix{
				inner,
				rows,
				columns
			}
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
}