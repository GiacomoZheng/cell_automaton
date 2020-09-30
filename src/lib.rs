mod screen;

mod states;

pub use states::Rule;

mod canvas;
pub use canvas::Canvas;
pub use canvas::CellAutomaton;
pub use canvas::BoardType::{Rect};

///
use std::sync::mpsc;
use std::sync::Arc;
use std::collections::HashMap;
use states::State;
pub struct Cell {
	state : Option<Arc<dyn State>>,

	/// true : +1, false : -1
	rx : mpsc::Receiver<bool>,

	/// true : +1, false : -1
	/// HashMap< index of an adjacent cell, tx >, with len as outdegree
	txs : HashMap<usize, mpsc::Sender<bool>>,

	tx_template : mpsc::Sender<bool>,

	count : usize,
}

impl Cell { // methods
	pub fn get_state(&self) -> bool {self.state.as_ref().expect("uninitialize").get_state()}

	/// listen
	pub fn recv_info(&mut self, multi : usize) {
		let change : isize = self.rx.iter().take(multi).map(|info| if info {1isize} else {-1isize}).sum();
		if change > 0 {
			self.count += change as usize;
		} else {
			self.count -= -change as usize;
		}
	}

	/// broadcast
	fn send_info(&mut self, info : bool) {//-> Result<(), &'static str> {
		for (_index, tx) in self.txs.iter_mut() {
			tx.send(info).unwrap();
			eprintln!("Send to {}", _index);
		}
	}

	pub fn update(&mut self) -> Result<bool, &'static str> {
		let (state, saw_info) = self.state.take().unwrap().update(self.count())?;
		
		self.state = Some(state);

		if let Some(info) = saw_info {
			self.send_info(info);
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// only for clicked button
	pub fn force_to(& mut self, on : bool) {
		// +
		let (state, saw_info) = self.state.take().unwrap().force_to(on);

		self.state = Some(state);

		if let Some(info) = saw_info {
			self.send_info(info);
		} else {
			panic!("no switch");
		}
		unimplemented!();
	}

	/// initialize
	pub fn set(&mut self, state : &Arc<dyn State>) -> bool {
		let on = state.as_ref().get_state();
		if on {
			self.send_info(true);
		} 
		self.state = Some(Arc::clone(state));
		on
	}

		
	/// index : the index of this cell in the board
	/// add the cells as in-adjacent
	pub fn push_adj(&self, index : usize, cell : &mut Cell) {
		if let Some(_) = cell.txs.insert(index, mpsc::Sender::clone(&self.tx_template)) {
			panic!("add an adj twice or there are two distinct adjs with the same index")
		}
	}

	pub fn append_adj(&self, v : Vec<(usize, &mut Cell)>) {
		for (index, cell) in v {
			self.push_adj(index, cell);
		}
	}
	
	pub fn count(&self) -> usize {
		self.count
	}

	/// for debug
	#[allow(dead_code)]
	pub fn count_adj(&self) -> usize {
		self.txs.len()
	}
}

impl Cell { 
	pub fn new() -> Cell {
		let (tx, rx) = mpsc::channel();
		Cell {
			state : None,
			tx_template : tx,
			txs : HashMap::new(),
			rx,
			count : 0,
		}
	}
}

use std::fmt;
impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}]", if self.get_state() {"■"} else {"□"})
    }
}
impl fmt::Debug for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}\t", self, self.count_adj()) // **
    }
}