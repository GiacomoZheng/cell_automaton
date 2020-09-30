use std::fmt;

use super::Cell;

pub trait Canvas : fmt::Debug + fmt::Display {
	// fn get_cell(&self, index : usize) -> Cell;
	fn count_cells_to_update(&self) -> usize;

	fn is_dead_at(&self, index : usize) -> bool;
	fn check_dead(&self, index : usize) {
		// I may comment it to speed up
		if self.is_dead_at(index) {
			panic!("try to activate a dead cell")
		}
	}
	fn activate_cell(&mut self, index : usize);
	fn kill_cell(&mut self, index : usize);
	fn count_cells(&self) -> usize;

	fn update_info(&mut self, index : usize);
	fn clear_info(&mut self);

	fn count_updating(&self) -> usize;

	fn update(&mut self) -> Result<(), &'static str>; // ?
}

// --------------------------------------------------
mod cell;
pub use cell::CellAutomaton;
pub use cell::BoardType; // ?



