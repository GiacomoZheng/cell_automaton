use super::Cell;

mod tools;
pub use tools::BoardType;
use tools::Layout;
use tools::BoardType::*;

/// main stage
pub struct Canvas {
	cells : Vec<Option<Cell>>,
	layout : Layout,
	/// for update
	/// 
	/// `None` : dead cell
	/// 
	/// `Some(None)` : live cell but no need to update
	/// 
	/// `Some(Some(n))` : live cell and need to update with n infos
	updating_vec : Vec<Option<Option<usize>>>,
}

impl Canvas { // constructure functions 
	pub fn new(m : usize, n : usize, height : usize) -> Canvas {
		let layout = Layout::new(m, n, height);
		let updating_vec : Vec<Option<Option<usize>>> = (0..layout.get_len()).map(|_| {Some(None)}).collect();
		Canvas {
			cells : Vec::new(),
			layout,

			updating_vec,
		}
	}
}

impl Canvas { // help functions
	fn is_dead_at(&self, index : usize) -> bool {
		self.updating_vec[index].is_none()
	}

	fn check_dead(&self, index : usize) {
		// I may comment it to speed up
		if self.is_dead_at(index) {
			panic!("try to activate a dead cell")
		}
	}

	/// mark the cell and update it in next generation
	fn active_cell(&mut self, index : usize) { // !
		self.check_dead(index);

		let saw = &mut self.updating_vec[index];
		if let Some(None) = saw {
			*saw = Some(Some(0));
		}
	}

	fn kill_cell(&mut self, index : usize) {
		self.updating_vec[index] = None;
	}

	// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
	fn update_vec(&mut self, index : usize) {
		self.active_cell(index);
		for e in self.layout.adjecents(index) {

			if !self.is_dead_at(e) {
				// + refine
				self.active_cell(e);
				if let Some(Some(multi)) = self.updating_vec.get_mut(e).unwrap() {
					*multi += 1; // ?
				}
			}
		}
	}

	fn clear_updating_vec(&mut self) {
		for pt in self.updating_vec.iter_mut() {
			*pt = match pt {
				Some(_) => Some(None),
				None => None
			}
		}
	}

	fn get_len(&self) -> usize {
		self.layout.get_len()
	}
}

use std::thread;
impl Canvas {
	pub fn update(&mut self) -> Result<(), &'static str> {
		let mut handles = Vec::new();
		
		for (index, &saw_info) in self.updating_vec.iter().enumerate() {
			// + refine
			if let Some(Some(multi)) = saw_info {
				let mut cell = self.cells[index].take().unwrap();
				// eprintln!("?{}:{:?}", index, self.updating_vec[index]);
				handles.push(thread::spawn(move || {
					cell.recv_info(multi);
					// eprintln!("!{}", index);
					(index, cell) // listen
				}));
			}
		}
		
		self.clear_updating_vec();
		
		let mut temp_vec = Vec::new();
		for handle in handles {
			temp_vec.push(handle.join().unwrap());
		}

		// --- all counters have been fixed here ---
		for (index, mut cell) in temp_vec {
			if cell.update()? {
				// add all adjecents into stack
				self.update_vec(index);
			}
			self.cells[index].replace(cell);
		}

		Ok(())
	}
}

use super::states::State;
use std::sync::Weak;
impl Canvas {
	// + refine
	pub fn init(&mut self, states : Vec<&Option<Weak<dyn State>>>) {//-> Result<(), &'static str> {
		self.clear_updating_vec();

		for index in 0..self.layout.get_len() {
			if let Some(mut cell) = self.cells[index].take() {

				if let Some(state) = states[index].as_ref() {
					self.active_cell(index);
					if cell.set(&state.upgrade().unwrap()) {
						self.update_vec(index);
					}
					
					self.cells[index].replace(cell);
				} else {
					self.cells[index] = None;
					self.kill_cell(index);
				}
			} // cell might be none depends on the type of cells
		}

		// Ok(())
	}
}

impl Canvas { // buildup
	pub fn set_type(&mut self, board_type : BoardType) {
		self.layout.set_type(board_type);
	}

	pub fn buildup_rect_board(&mut self) {
		self.set_type(Rect);
		self.buildup_board();
	}

	pub fn buildup_board(&mut self) {		
		// + the dead cells
		match self.layout.get_type() { // buildup the adjecents
			
			Rect => {
				// + refine
				for _ in 0.. self.get_len() {
					self.cells.push(Some(Cell::new()));
				}
				for i in 0..self.get_len() {
					// !!!!
					let vec_index : Vec<usize> = self.layout.adjecents(i);
					let mut vec_cell : Vec<Cell> = vec_index.iter().map(|&index| {self.cells[index].take().unwrap()}).collect();
					eprintln!("indexs: {:?}", vec_index);
					eprintln!("cells: {:?}", vec_cell.len());
					// let mut vec_cell : Vec<(usize, Cell)> = vec_index.iter().map(|&index| {(index, self.cells[index].take().unwrap())}).collect();

					self.cells.get_mut(i).unwrap().as_mut().unwrap().append_adj(vec_index.iter().map(|&index| {index}).zip(vec_cell.iter_mut()).collect());
					
					for (index, cell) in vec_index.into_iter().zip(vec_cell.into_iter()) {
						self.cells[index].replace(cell);
					}
				}
			},

			UnInitialized => panic!("undefined right now")
		}
	}
}

impl Canvas {// show infomations of this canvas
	pub fn count_updating(&self) -> usize {
		self.updating_vec.iter().filter(|option| if let Some(Some(_)) = option {true} else {false}).count()
	}
}

use std::fmt;
impl fmt::Debug for Canvas {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut iterator = self.cells.iter();
		for z in 0..self.layout.get_height() {
			writeln!(f, "level {}", z)?;

			for y in 0..self.layout.y_bound() {
				for x in 0..self.layout.x_bound() {
					// eprint!("{} ", self.layout.index_of(x, y, z)); // !
					write!(f, "{}", match iterator.next().unwrap() {
						Some(cell) => format!("{}:{:?}",
							// + refine
							match self.updating_vec[self.layout.index_of(x, y, z)] {
								Some(Some(multi)) => multi.to_string(),
								_ => String::from(" "),
							},
							cell),
						None => String::from("\t")}
					)?;
				}
				write!(f, "\n")?;
			}
		}
		write!(f, "")
    }
}
impl fmt::Display for Canvas {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// let mut iterator = self.cells.iter().;
		for y in 0..self.layout.y_bound() {
			for x in 0..self.layout.x_bound() {
				write!(f, "{}", match self.cells.get(self.layout.sceen_index_of(x, y)).unwrap() { Some(cell) => format!("{}", cell), None => String::from("\t")})?;
			}
			write!(f, "\n")?;
		}
		write!(f, "")
    }
}

