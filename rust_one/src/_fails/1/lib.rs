#[derive(Debug)]
pub struct Cell {
	state: &'static Box<dyn State>,
	adjecents: Vec<Box<Cell>>
}

impl Cell {
	pub fn new(&self, state : &'static Box<dyn State>, adjecents : Vec<Box<Cell>>) -> Cell {
		Cell {
			state,
			adjecents
		}
	}
}

impl Cell {
	fn get_state(&self) -> bool {
		self.state.get_state()
	}

	fn count(&self) -> usize {
		let mut counter : usize = 0;
		for state in self.adjecents.iter() {
			if state.get_state() {
				counter += 1;
			}
		}
		counter
	}

	pub fn update(&mut self) -> Result<(), &str> {
		let counter = self.count();
		self.state = self.state.update(counter)?;
		Ok(())
	}
}

// ? pub
pub trait State : std::fmt::Debug {
	fn get_state(&self) -> bool;
	fn get_rule(&self) -> &Vec<&'static Box<dyn State>>;

	fn update(& self, n : usize) -> Result<&'static Box<dyn State>, &str> {
		match self.get_rule().get(n) {
			Some(b) => Ok(*b),
			None => panic!("Error in updating states")
		}
	}
}

#[derive(Debug)]
pub struct Off {
	// ?
	pub rule : Vec<&'static Box<dyn State>>
}
impl State for Off {
	fn get_state(&self) -> bool {
		false
	}

	fn get_rule(&self) -> &Vec<&'static Box<dyn State>> {
		&self.rule
	}
}
impl Off {
	pub fn new() -> Off {
		Off { rule : Vec::new() }
	}

	pub fn from(rule : Vec<&'static Box<dyn State>>) -> Off {
		Off { rule }
	}
}

#[derive(Debug)]
pub struct On {
	// ?
	pub rule : Vec<&'static Box<dyn State>>
}
impl State for On {
	fn get_state(&self) -> bool {
		true
	}

	fn get_rule(&self) -> &Vec<&'static Box<dyn State>> {
		&self.rule
	}
}
impl On {
	pub fn new() -> On {
		On { rule : Vec::new() }
	}

	pub fn from(rule : Vec<&'static Box<dyn State>>) -> On {
		On { rule }
	}
}
