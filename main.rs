
use rand::Rng;

#[derive(Debug)]
struct Graph{
	nodes:usize,
	matrix:Vec<Vec<bool>>
}
impl Graph{
	fn new(nodes:usize) -> Graph{
		let mut rng = rand::thread_rng();
		
		let mut matrix = Vec::new();

		for _ in 0..nodes{
			let mut row = Vec::new();
			for _ in 0..nodes{
				row.push(rng.gen());
			}
			matrix.push(row);
		}
		
		Graph{nodes,matrix}
	}
	fn print_dot(&self){
		println!("digraph matrix {{");
		for (from,line) in self.matrix.iter().enumerate(){
			for (to,connected) in line.iter().enumerate(){
				if *connected{
					println!("\t{} -> {};",from,to);
				}
			}
		}
		println!("}}");
	}
	fn print_matrix(&self){
		print!("  ");//corner
		for i in 0..self.nodes{
			print!("{} ",i);//nodes row
		}
		println!("");//separator
		for (index,row) in self.matrix.iter().enumerate() {
			print!("{} ",index);//nodes column
			for cell in row{
				print!("{} ",if *cell {1} else {0});
			}
			println!("")
		}
	}

}


struct K2Tree{
	value:Option<bool>,
	nodes:Option<Vec<K2Tree>>
}

impl K2Tree{
	fn new(matrix:&Vec<Vec<bool>>,size:usize){


	}
	fn distribute(matrix:&Vec<Vec<bool>>,size:usize)->Vec<K2Tree>{

		Vec::new()
	}
	fn build(matrix:&Vec<Vec<bool>>,size:usize)->K2Tree{

		let mut acc:usize = 0;
		let mut elements:usize=0;
		for row in matrix{
			for cell in row{
				elements+=1;
				if *cell {acc+=1};
			}
		}
		return match acc {
			0 => K2Tree{value:Some(false),nodes:None},
			_ if acc == elements => K2Tree{value: Some(true),nodes:None},
			_ => K2Tree{value:None,nodes:Some(K2Tree::distribute(matrix,size))}
		}
	}

}

fn main() {
	let graph = Graph::new(6);
	//graph.print_dot()
	graph.print_matrix();

}
