use std::sync::mpsc;
use std::sync::Arc;

use super::states::State;

pub struct Cell {
	state : Option<Arc<dyn State>>,

	/// true : +1, false : -1
	rx : mpsc::Receiver<bool>,

	/// true : +1, false : -1
	txs : Vec<mpsc::Sender<bool>>,
	tx_template : mpsc::Sender<bool>,

	count : usize,
}
impl Cell { // methods
	pub fn get_state(&self) -> bool {self.state.as_ref().expect("uninitialize").get_state()}

	/// listen
	pub fn recv_infor(&mut self, multi : usize) {
		// + refine
		for _ in 0..multi {
			match self.rx.recv().unwrap() {
				true  => {self.count += 1;},
				false => {self.count -= 1;},
			};
		}
	}

	/// broadcast
	fn send_infor(&mut self, infor : bool) {//-> Result<(), &'static str> {
		for tx in self.txs.iter_mut() {
			tx.send(infor).unwrap();
		}
	}

	pub fn update(&mut self) -> Result<bool, &'static str> {
		let (state, saw_infor) = self.state.take().unwrap().update(self.count())?;
		
		self.state = Some(state);

		if let Some(infor) = saw_infor {
			self.send_infor(infor);
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// only for clicked button
	pub fn force_to(& mut self, on : bool) {
		// +
		let (state, saw_infor) = self.state.take().unwrap().force_to(on);

		self.state = Some(state);

		if let Some(infor) = saw_infor {
			self.send_infor(infor);
		} else {
			panic!("no switch");
		}
	}

	/// initialize
	pub fn set(&mut self, state : &Arc<dyn State>) -> bool {
		// eprintln!("!{}", state.get_state()); // !
		let on = state.as_ref().get_state();
		if on {
			self.send_infor(true);
		} 
		self.state = Some(Arc::clone(state));
		on
	}

	pub fn append_adj(& mut self, v : Vec<&mut Cell>) {
		for cell in v {
			cell.txs.push(mpsc::Sender::clone(&self.tx_template));
		}
	}
	
	pub fn count(&self) -> usize {
		self.count
	}

	/// for debug
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
			txs : Vec::new(),
			rx,
			count : 0,
		}
	}
	
	pub fn from(saw_state : &Option<Arc<dyn State>>) -> Result<Cell, &'static str> {
		if let Some(state) = saw_state.as_ref() {
			let mut cell = Cell::new();
			cell.set(state);
			Ok(cell)
		} else {
			Err("wrong initialization")
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
		write!(f, "{}:{}\t", self, self.count()) // **
    }
}