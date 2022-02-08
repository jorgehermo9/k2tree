
pub mod iter; 

use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct Sequence<T>{
	//space worst case: 2*O(n)
	data: Vec<T>,
	target:T,
	target_index:Vec<usize>
}

impl <T> Sequence<T> where T:Eq{
	pub fn new(target:T) -> Sequence<T>{
		Sequence{
			data:Vec::new(),
			target,
			target_index: Vec::new()
		}
	}
	pub fn from_iter(target:T,iter:impl IntoIterator<Item=T>)-> Self{
		let mut seq = Self::new(target);
		for element in iter{
			seq.push(element);
		}
		seq
	}
	pub fn rank(&self,i:usize)->Option<usize>{
		//O(1)
		return self.target_index.get(i).copied();
	}
	pub fn select(&self,j:usize)->Option<usize>{
		//O(n)
		self.target_index.iter().find(|&&e| e == j).copied()
	}
	pub fn push(&mut self,item:T){
		let prev_index = *self.target_index.last().unwrap_or(&0);
		if item == self.target {
			self.target_index.push(prev_index+1);
		}else{
			self.target_index.push(prev_index);
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
		let seq = Sequence::from_iter(None,vec);
		assert_eq!(seq.rank(2).unwrap(),2);
		assert_eq!(seq.select(2).unwrap(),1);

	}
}