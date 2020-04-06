use super::cells::Cell;

/// ```
/// Layout(type, m : usize, n : usize)
/// ```
pub struct Layout(&'static str, usize, usize);

impl Layout {
	#[allow(non_snake_case)]
	pub fn set_type(&mut self, Type : &'static str) {
		self.0 = Type;
	}

	pub fn len(&self) -> usize {
		self.1 * self.2
	}

	pub fn x_bound(&self) -> usize {
		self.1
	}
	pub fn y_bound(&self) -> usize {
		self.2
	}

	pub fn new(m : usize, n : usize) -> Layout {
		Layout("", m, n)
	}
}

#[allow(dead_code)]
impl Layout { // basic for index
	pub fn index_of(&self, x : usize, y : usize) -> usize {x + y * self.1}

	fn above_one(&self, index : usize) -> usize {index - self.1}
	fn below_one(&self, index : usize) -> usize {index + self.1}
	#[allow(non_snake_case)]
	fn left__one(&self, index : usize) -> usize {index -      1}
	fn right_one(&self, index : usize) -> usize {index +      1}

	fn la_one(&self, index : usize) -> usize {index - self.1 - 1}
	fn lb_one(&self, index : usize) -> usize {index + self.1 - 1}
	fn ra_one(&self, index : usize) -> usize {index - self.1 + 1}
	fn rb_one(&self, index : usize) -> usize {index + self.1 + 1}
}

impl Layout { // for different types
	// + input a index and output all the adjecents' indexes
	pub fn adjecents(&self, index : usize) -> Vec<usize> {
		let x = index % self.1; // m
		let y = index / self.1; // m
		match self.0 {
			"rect" => if x == 0 {
					if y == 0 {
						vec![self.right_one(index), self.below_one(index)]
					} else if y < self.2 - 1 {
						vec![self.right_one(index), self.below_one(index), self.above_one(index)]
					} else {
						vec![self.right_one(index), self.above_one(index)]
					}
				} else if x < self.1 - 1 {
					if y == 0 {
						vec![self.right_one(index), self.below_one(index), self.left__one(index)]
					} else if y < self.2 - 1 {
						vec![self.right_one(index), self.below_one(index), self.above_one(index), self.left__one(index)]
					} else {
						vec![self.right_one(index), self.above_one(index), self.left__one(index)]
					}
				} else {
					if y == 0 {
						vec![self.below_one(index), self.left__one(index)]
					} else if y < self.2 - 1 {
						vec![self.below_one(index), self.above_one(index), self.left__one(index)]
					} else {
						vec![self.above_one(index), self.left__one(index)]
					}
				},
			_ => panic!("undefined right now")
		}
	}
}


/// main stage
pub struct Canvas {
	pub board : Vec<Option<Cell>>,
	pub layout : Layout,
	/// for update
	pub cell_stack : Vec<usize>, // Vec < multiplicity >
}

impl Canvas { // constructure functions 
	pub fn new(m : usize, n : usize) -> Canvas {
		let layout = Layout::new(m, n);
		Canvas {
			board : Vec::new(),
			layout,
			cell_stack : Vec::new(),
		}
	}
}
use std::thread;
impl Canvas {
	pub fn update(&mut self) -> Result<(), &'static str> {
		// + refine
		let mut handles = Vec::new();
		
		// eprintln!("cell_stack:{:?}", self.cell_stack); // !
		for (index, &multi) in self.cell_stack.iter().enumerate() {//.filter(|(_, &multi)| {multi != 0}) { // + refine
			let mut cell = self.board[index].take().unwrap();
			handles.push(thread::spawn(move || {
				cell.recv_infor(multi);
				(index, cell) // listen
			}));
		}
		
		self.clear_stack();
		
		let mut temp_vec = Vec::new();
		for handle in handles {
			temp_vec.push(handle.join().unwrap());
		}

		// --- all counters have been fixed here ---
		for (index, mut cell) in temp_vec {
			if cell.update()? {
				// add all adjecents into stack
				self.update_stack(index);
			}
			self.board[index].replace(cell);
		}

		Ok(())
	}
}

impl Canvas { // help functions
	fn update_stack(&mut self, index : usize) {
		for e in self.layout.adjecents(index) {
			*self.cell_stack.get_mut(e).unwrap() += 1;
		}
		// eprintln!("cell_stack:{:?}", self.cell_stack);
	}

	fn clear_stack(&mut self) {
		self.cell_stack.clear();
		for _ in 0..self.len() {
			self.cell_stack.push(0);
		}
	}

	fn len(&self) -> usize {
		self.layout.len()
	}
}

use super::states::State;
use std::sync::Weak;
impl Canvas {
	// + refine
	pub fn init(&mut self, states : Vec<&Option<Weak<dyn State>>>) -> Result<(), &'static str> {
		self.clear_stack();

		for index in 0..self.layout.len() {
			if let Some(cell) = self.board[index].as_mut() {

				if let Some(state) = states[index].as_ref() {
					if cell.set(&state.upgrade().unwrap()) {
						self.update_stack(index);
					}

				} else {
					return Err("wrong initialization : need more configs");
				}
			} // cell might be none depends on the type of board
		}

		Ok(())
	}
}

impl Canvas { // buildup
	pub fn buildup_rect_board(& mut self) {
		self.layout.set_type("rect");

		for _ in 0.. self.len() {
			self.board.push(Some(Cell::new()));
		}
		
		{ // buildup the adjecents

			for i in 0..self.len() { // + refine
				let vec_index : Vec<usize> = self.layout.adjecents(i);
				let mut vec_cell : Vec<Cell> = vec_index.iter().map(|&index| {self.board[index].take().unwrap()}).collect();

				self.board.get_mut(i).unwrap().as_mut().unwrap().append_adj(vec_cell.iter_mut().collect());
				
				for (index, cell) in vec_index.into_iter().zip(vec_cell.into_iter()) {
					self.board[index].replace(cell);
				}
			}
		}
	}
}

use std::fmt;
impl fmt::Debug for Canvas {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut iterator = self.board.iter();
		for y in 0..self.layout.y_bound() {
			for x in 0..self.layout.x_bound() {
				// eprint!("{} ", self.layout.index_of(x, y)); // !
				write!(f, "{}", match iterator.next().unwrap() { Some(cell) => format!("{}:{:?}", self.cell_stack[self.layout.index_of(x, y)], cell), None => String::from("\t")})?;
			}
			write!(f, "\n")?;
		}
		write!(f, "")
    }
}
impl fmt::Display for Canvas {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut iterator = self.board.iter();
		for _ in 0..self.layout.y_bound() {
			for _ in 0..self.layout.x_bound() {
				write!(f, "{}", match iterator.next().unwrap() { Some(cell) => format!("{}", cell), None => String::from("\t")})?;
			}
			write!(f, "\n")?;
		}
		write!(f, "")
    }
}

