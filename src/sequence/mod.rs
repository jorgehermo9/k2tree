
pub mod iter; 



#[derive(Debug)]
pub struct Sequence<T>{
	//space worst case: 2*O(n)
	data: Vec<T>,
	target:T,
	target_index:Vec<(usize,usize)>//index,occurences
}

impl <T> Sequence<T> where T:Eq{
	pub fn new(data: Vec<T>, target:T) -> Sequence<T>{
		let mut target_index = Vec::new();
		let mut occurrences=0;
		for (index,element) in data.iter().enumerate() {
			if *element == target{
				occurrences+=1;
				target_index.push((index,occurrences));
			}
		}
		Sequence{
			data,
			target,
			target_index
		}
	}
	//Otra implementacion en tiempo O(1) sería para cada posición
	//del array data, guardar el número de ocurrencias anteriores del target
	//Pero esto tendría complejidad espacial O(n) 
	pub fn rank(&self,i:usize)->Option<usize>{
		if i >= self.data.len(){return None};
		match self.target_index.iter().position(|&(index,_)| index > i){
			None=>Some(self.target_index.last().unwrap().1),
			Some(0)=>None,//If first index, there is no occurrence previous to i
			Some(index) => Some(self.target_index[index-1].1)//Return occurences for the previous position
		}
	}
	pub fn select(&self,j:usize)->Option<usize>{
		//O(n)
		match self.target_index.get(j){
			None => None,
			Some(&(index,_))=>Some(index)
		}
	}
	pub fn push(&mut self,item:T){
		if item == self.target {
			let previous_occurrences = match self.target_index.last(){
				None=>0,
				Some((_,occurrences))=>*occurrences
			};
			self.target_index.push((self.data.len(),previous_occurrences+1));
		}
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
	pub fn get_target_index(&self)->&Vec<(usize,usize)>{
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