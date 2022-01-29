
pub mod iter; 



#[derive(Debug)]
pub struct Sequence<T>{
	//space worst case: 2*O(n)
	data: Vec<T>,
	target:T,
	target_index:Vec<usize>
}

impl <T> Sequence<T> where T:Eq{
	pub fn new(data: Vec<T>, target:T) -> Sequence<T>{
		let mut target_index = Vec::new();
		for (index,element) in data.iter().enumerate() {
			if *element == target{
				target_index.push(index);
			}
		}
		Sequence{
			data,
			target,
			target_index
		}
	}
	pub fn rank(&self,i:usize)->Option<usize>{
		//O(n)
		if i >= self.data.len(){return None};
		let indexes = self.target_index.iter().take_while(|index| **index<=i);
		Some(indexes.count())

	}
	pub fn select(&self,j:usize)->Option<usize>{
		//O(1)
		self.target_index.get(j-1).copied()
	}
	pub fn push(&mut self,item:T){
		if item == self.target {
			self.target_index.push(self.data.len());
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
		assert_eq!(seq.rank(2).unwrap(),2);
		assert_eq!(seq.select(2).unwrap(),1);

	}
}