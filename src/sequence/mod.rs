
pub mod iter; 


//Otra opcion de implementacion con menor complejidad espacial
//Guardar en un vector sólo las posiciones en las que hay un None.
//El rank se haría en el peor caso en O(n)
//sería -> target_index.iter().filter(|index| index<=i ).len()
#[derive(Debug)]
pub struct Sequence<T>{
	data: Vec<T>,
	target:T,
	target_index:Vec<usize>
}

impl <T> Sequence<T> where T:Eq{
	pub fn new(data: Vec<T>, target:T) -> Sequence<T>{
		let mut target_index = Vec::new();
		let mut acc = 0;

		for element in &data{
			if *element == target{
				acc+=1;
			}
			target_index.push(acc)
		}
		Sequence{
			data,
			target,
			target_index
		}
	}
	pub fn rank(&self,i:usize)->Option<&usize>{
		//O(1)
		//checking for i in range decreases performance
		//if i >= self.data.len(){panic!("index out of range")};
		self.target_index.get(i)
	}
	pub fn select(&self,j:usize)->Option<usize>{
		//O(n)
		self.target_index.iter().position(|occ| *occ ==j)
	}
	pub fn push(&mut self,item:T){
		let previous_rank = self.target_index.last().unwrap_or(&0);
		let current_rank = if item==self.target{*previous_rank+1} else{*previous_rank};
		self.target_index.push(current_rank);
		self.data.push(item);
	}
	pub fn get(&self,index:usize)->Option<&T>{
		self.data.get(index)
	}
	pub fn len(&self)->usize{
		self.data.len()
	}
	pub fn iter(&self)->std::slice::Iter<T>{
		return self.into_iter()
	}
	pub fn iter_mut(&mut self)-> std::slice::IterMut<T>{
		return self.into_iter()
	}
	pub fn get_data(&self)->&Vec<T>{
		&self.data
	}
	pub fn get_target_index(&self)->&Vec<usize>{
		&self.target_index
	}
}

#[cfg(test)]
mod tests{
	use super::Sequence;
	#[test]
	fn it_works(){
		let vec = vec![None,None,Some(1),Some(2),None];
		let seq = Sequence::new(vec,None);
		println!("{:?}",seq)
	}
}