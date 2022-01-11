pub mod matrix{
	
	pub struct Matrix<T> where T:Clone{
		inner:Vec<Vec<T>>,
		rows:usize,
		columns:usize
	}
	
	
	impl <T>Matrix<T> where T:Clone{
		pub fn new(rows:usize,columns:usize,default:T) -> Matrix<T>{
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
		pub fn get(&self,i:usize,j:usize) ->&T{
			&self.inner[i][j]
		}
		pub fn set(&mut self,i:usize,j:usize,value:T){
			self.inner[i][j] = value;
		}
		pub fn get_columns(&self) -> usize{
			self.columns
		}
		pub fn get_rows(&self) -> usize{
			self.rows
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}