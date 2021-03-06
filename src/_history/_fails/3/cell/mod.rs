use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;

// #[derive(Debug)]
pub struct Cell {
	state: Option<Rc<dyn State>>,
	adjecents: Vec<Weak<RefCell<Cell>>>
}
impl Cell {
	fn get_state(&self) -> bool {
		self.state.as_ref().expect("uninitialize").get_state()
	}

	fn count(&self) -> usize {
		self.adjecents.iter().filter(|cell| {cell.upgrade().unwrap().borrow().get_state()}).count()
	}

	pub fn update(&mut self) -> Result<(), &str> {
		self.state = Some(self.state.take().expect("uninitialize").update(self.count())?);
		Ok(())
	}

	pub fn append_adj(&mut self, v : &mut Vec<Weak<RefCell<Cell>>>) { // ?
		self.adjecents.append(v)
	}
}
impl Cell {
	pub fn longlive() -> Cell {
		Cell {
			state : Some(Rc::new( On::from(vec![true , true , true , true , true]))),
			adjecents : Vec::new()
		}
	}
	pub fn longdead() -> Cell {
		Cell {
			state : Some(Rc::new(Off::from(vec![false, false, false, false, false]))),
			adjecents : Vec::new()
		}
	}
}
impl Default for Cell {
    fn default() -> Self { Cell::longdead() }
}
impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]:{}", if self.get_state() {"■"} else {"□"}, self.adjecents.iter().count())
    }
}

trait State : fmt::Debug {
	fn get_state(&self) -> bool;
	fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str>;
}

#[derive(Debug)]
pub struct Off {
	rule : Vec<bool>,
	pub turn_on : Option<RefCell<Weak<On>>>
}
impl Off {
	pub fn new() -> Off {
		Off::from(Vec::new())
	}

	pub fn from(v : Vec<bool>) -> Off {
		Off {
			rule : v,
			turn_on : None
		}
	}
}
impl State for Off {
	fn get_state(&self) -> bool {false}

	fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str> {
		match self.rule.get(n) {
			Some(b) => if *b {
				Ok(self.turn_on.as_ref().expect("uninitialize").borrow().upgrade().expect("dropped"))
			} else {
				Ok(self)
			},
			None => Err("number of adjecents is too big")
		}
	}
}

#[derive(Debug)]
pub struct On {
	rule : Vec<bool>,
	pub turn_off : Option<RefCell<Weak<Off>>>
}
impl On {
	pub fn new() -> On {
		On::from(Vec::new())
	}

	pub fn from(v : Vec<bool>) -> On {
		On {
			rule : v,
			turn_off : None
		}
	}
}
impl State for On {
	fn get_state(&self) -> bool {true}

	fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str> {
		match self.rule.get(n) {
			Some(&b) => if b {
				Ok(self)
			} else {
				Ok(self.turn_off.as_ref().expect("uninitialize").borrow().upgrade().expect("dropped"))
			},
			None => Err("number of adjecents is too big")
		}
	}
}
