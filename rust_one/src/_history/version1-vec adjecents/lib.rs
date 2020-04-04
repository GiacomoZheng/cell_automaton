pub mod tools;

mod canvas;
pub use canvas::Canvas;

mod states;
pub use states::Cell;

use states::{On, Off};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;

pub struct Rule {
	on  : RefCell<Option<Rc<On>>>,
	off : RefCell<Option<Rc<Off>>>,

	on_config  : RefCell<Weak<Config>>,
	off_config : RefCell<Weak<Config>>
}
impl Rule {
	pub fn from(vec_on : Vec<bool>, vec_off : Vec<bool>) -> Rc<Rule> {
		let rule = Rule {on : RefCell::new(None), off : RefCell::new(None), on_config : RefCell::new(Weak::new()), off_config : RefCell::new(Weak::new())};
		let rc = Rc::new(rule);
		let on = Rc::new(On {
			// rule : Rc::downgrade(&Rc::clone(&rc)),
			rule : Rc::downgrade(&rc),
			mapping : vec_on
		});
		let off = Rc::new(Off {
			// rule : Rc::downgrade(&Rc::clone(&rc)),
			rule : Rc::downgrade(&rc),
			mapping : vec_off
		});
		rc.as_ref().on.borrow_mut().replace(on);
		rc.as_ref().off.borrow_mut().replace(off);
		rc
	}

	pub fn count(&self, on : bool) -> usize {
		if on {
			Rc::strong_count(&self.on.borrow().as_ref().unwrap()) - 1
		} else {
			Rc::strong_count(&self.off.borrow().as_ref().unwrap()) - 1
		}
	}
}
impl fmt::Debug for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}, {:?}", self.on, self.off)
    }
}

#[derive(Debug)]
pub struct Config {
	pub on : bool,
	pub rule : Rc<Rule>
}
impl Rule {
	pub fn on(self : &Rc<Self>) -> Rc<Config> {
		let mut borrow = self.on_config.borrow_mut();
		if let Some(config) = borrow.upgrade() {
			Rc::clone(&config)
		} else {
			let pt = Rc::new(Config {
				on : true,
				rule : Rc::clone(self)
			});
			*borrow = Rc::downgrade(&pt);
			pt
		}
	}
	pub fn off(self : &Rc<Self>) -> Rc<Config> {
		let mut borrow = self.off_config.borrow_mut();
		if let Some(config) = borrow.upgrade() {
			Rc::clone(&config)
		} else {
			let pt = Rc::new(Config {
				on : false,
				rule : Rc::clone(self)
			});
			*borrow = Rc::downgrade(&pt);
			pt
		}
	}

}

