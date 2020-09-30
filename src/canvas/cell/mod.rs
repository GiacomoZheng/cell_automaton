use std::sync::Weak;

use super::super::states::State;
use super::*;

/// main stage
pub struct CellAutomaton {
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

impl CellAutomaton { // constructure functions 
	pub fn new(m : usize, n : usize, height : usize) -> CellAutomaton {
		let layout = Layout::new(m, n, height);
		let updating_vec : Vec<Option<Option<usize>>> = (0..layout.get_len()).map(|_| {Some(None)}).collect();
		CellAutomaton {
			cells : Vec::new(),
			layout,

			updating_vec,
		}
	}
}

use std::thread;

impl Canvas for CellAutomaton { 
	// fn get_cell(&self, index : usize) -> Cell {
	// 	self.cells[index]
	// }

	fn count_cells_to_update(&self) -> usize {
		self.updating_vec.iter().filter(|option| if let Some(Some(_)) = option {true} else {false}).count()
	}

	fn is_dead_at(&self, index : usize) -> bool {
		self.updating_vec[index].is_none()
	}

	/// mark the cell and update it in next generation
	fn activate_cell(&mut self, index : usize) { // !
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
	fn update_info(&mut self, index : usize) {
		eprintln!("\tfrom {}", index);
		self.activate_cell(index);
		for e in self.layout.adjecents(index) {

			if !self.is_dead_at(e) {
				// + refine
				self.activate_cell(e);
				if let Some(Some(multi)) = self.updating_vec.get_mut(e).unwrap() {
					*multi += 1; // ?
				}
			}
		}
	}

	fn clear_info(&mut self) {
		for pt in self.updating_vec.iter_mut() {
			*pt = match pt {
				Some(_) => Some(None),
				None => None
			}
		}
	}

	fn count_cells(&self) -> usize {
		self.layout.get_len()
	}

	fn count_updating(&self) -> usize {
		self.updating_vec.iter().filter(|option| if let Some(Some(_)) = option {true} else {false}).count()
	}


	fn update(&mut self) -> Result<(), &'static str> {
		// + refine: I need to compare the times
		let mut handles = Vec::new();
		
		for (index, &saw_info) in self.updating_vec.iter().enumerate() {
			// + refine
			if let Some(Some(multi)) = saw_info {
				let mut cell = self.cells[index].take().unwrap();
				handles.push(thread::spawn(move || {
					cell.recv_info(multi);
					// eprintln!("!{}", index);
					(index, cell) // listen
				}));
			}
		}
		
		self.clear_info();
		
		let mut temp_vec = Vec::new();
		for handle in handles {
			temp_vec.push(handle.join().unwrap());
		}

		// --- all counters have been fixed here ---
		for (index, mut cell) in temp_vec {
			if cell.update()? {
				// add all adjecents will be activiated
				self.update_info(index);
			}
			self.cells[index].replace(cell);
		}

		Ok(())
	}
}

impl CellAutomaton {
	// + refine
	pub fn init(&mut self, states : Vec<&Option<Weak<dyn State>>>) {//-> Result<(), &'static str> {
		self.clear_info();

		for index in 0..self.layout.get_len() {
			if let Some(mut cell) = self.cells[index].take() {

				if let Some(state) = states[index].as_ref() {
					self.activate_cell(index);
					if cell.set(&state.upgrade().unwrap()) {
						self.update_info(index);
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


mod tools;
use tools::Layout;
pub use tools::BoardType::{self, *};

impl CellAutomaton { // buildup
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
				for _ in 0.. self.count_cells() {
					self.cells.push(Some(Cell::new()));
				}
				for i in 0..self.count_cells() {
					// !!!!
					let cell = self.cells[i].take();
					if let Some(cell) = cell {
						// eprintln!("indexs of {}: {:?}", i, self.layout.adjecents(i));
						for index in self.layout.adjecents(i).into_iter() {
							cell.push_adj(i, self.cells[index].as_mut().unwrap());
						}
						self.cells[i].replace(cell);
					}
				}
			},

			UnInitialized => panic!("undefined right now")
		}
	}
}

use std::fmt;
impl fmt::Debug for CellAutomaton {
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

impl fmt::Display for CellAutomaton {
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