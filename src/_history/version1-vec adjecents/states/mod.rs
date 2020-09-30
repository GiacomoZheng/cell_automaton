use super::Rule;
use std::fmt;
use std::rc::Weak;

pub struct On {
	pub rule : Weak<Rule>,
	pub mapping : Vec<bool>
}
impl fmt::Debug for On {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "on : {:?}", self.mapping)
	}
}

pub struct Off {
	pub rule : Weak<Rule>,
	pub mapping : Vec<bool>
}
impl fmt::Debug for Off {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "on : {:?}", self.mapping)
	}
}

use std::rc::Rc;
trait State : fmt::Debug {
	fn get_state(&self) -> bool;
	fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str>; // +

	/// for botton clicked in gui 
	fn force_to(self : Rc<Self>, on : bool) -> Rc<dyn State>;
}

impl State for On {
	fn get_state(&self) -> bool {true}

	fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str> {
		if let Some(b) = self.mapping.get(n) {
			if *b {
				Ok(self)
			} else {
				Ok(self.rule.upgrade().unwrap().off.borrow().as_ref().unwrap().clone())
			}
		} else {
			Err("too many adjecents around")
		}
	}

	fn force_to(self : Rc<Self>, on : bool) -> Rc<dyn State> {
		if on {
			self
		} else {
			self.rule.upgrade().unwrap().off.borrow().as_ref().unwrap().clone()
		}
	}
}

impl State for Off {
	fn get_state(&self) -> bool {false}

	fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str> {
		if let Some(b) = self.mapping.get(n) {
			if *b {
				Ok(self.rule.upgrade().unwrap().on.borrow().as_ref().unwrap().clone())
			} else {
				Ok(self)
			}
		} else {
			Err("too many adjecents around")
		}
	}

	fn force_to(self : Rc<Self>, on : bool) -> Rc<dyn State> {
		if on {
			self.rule.upgrade().unwrap().on.borrow().as_ref().unwrap().clone()
		} else {
			self
		}
	}
}

use std::cell::RefCell;
pub struct Cell {
	state : Option<Rc<dyn State>>,
	adjecents : Vec<Weak<RefCell<Cell>>>,

	count : usize,
}

impl Cell { // methods
	pub fn get_state(&self) -> bool {self.state.as_ref().expect("uninitialize").get_state()}

	pub fn update(& mut self) -> Result<(), &'static str> {
		self.state = Some(self.state.take().unwrap().update(self.count())?);
		Ok(())
	}

	/// only for clicked button
	pub fn force_to(& mut self, on : bool) {
		self.state = Some(self.state.take().unwrap().force_to(on));
	}

	/// init it after buildup board
	pub fn init(& mut self) {
		self.count = self.adjecents.iter().filter(|cell| cell.upgrade().unwrap().borrow().get_state()).count();
	}

	pub fn count(&self) -> usize {
		self.count
	}

	// initialize
	pub fn set(& mut self, config : &Config) {
		if config.on {
			self.state = Some(Rc::clone(&config.rule.on.borrow().as_ref().unwrap()) as Rc<dyn State>);
		} else {
			self.state = Some(Rc::clone(&config.rule.off.borrow().as_ref().unwrap()) as Rc<dyn State>);
		}
	}
	
	/// for debug
	pub fn count_adj(&self) -> usize {
		self.adjecents.len()
	}

	pub fn append_adj(& mut self, v : & mut Vec<Weak<RefCell<Cell>>>) {
		self.adjecents.append(v);
	}
}

use super::Config;
impl Cell { 
	pub fn new() -> Cell {
		Cell {
			state : None,
			adjecents : Vec::new(),
			count : 0
		}
	}
	
	pub fn from(config : &Option<Config>) -> Result<Cell, &'static str> {
		if let Some(c) = config {
			let mut cell = Cell::new();
			cell.set(c);
			Ok(cell)
		} else {
			Err("wrong initialization")
		}
	}
}

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