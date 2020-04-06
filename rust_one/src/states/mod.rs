use std::sync::{Arc, Weak};
use std::fmt;

struct On {
	rule : Weak<Rule>,
	mapping : Vec<bool>
}
struct Off {
	rule : Weak<Rule>,
	mapping : Vec<bool>
}

impl fmt::Debug for On {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "on : {:?}", self.mapping)
	}
}
impl fmt::Display for On {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut res = String::new();
		for (count, &b) in self.mapping.iter().enumerate() {
			if b {
				res += count.to_string().as_str();
			}
		}
		write!(f, "S{}", res)
	}
}

impl fmt::Debug for Off {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "on : {:?}", self.mapping)
	}
}
impl fmt::Display for Off {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut res = String::new();
		for (count, &b) in self.mapping.iter().enumerate() {
			if b {
				res += count.to_string().as_str();
			}
		}
		write!(f, "B{}", res)
	}
}

use std::marker::{Sync, Send};
pub trait State : fmt::Debug + fmt::Display + Sync + Send {
	fn get_state(&self) -> bool;
	fn update(self : Arc<Self>, n : usize) -> Result<(Arc<dyn State>, Option<bool>), &'static str>;

	/// for botton clicked in gui 
	fn force_to(self : Arc<Self>, on : bool) -> (Arc<dyn State>, Option<bool>);

}

impl State for On {
	fn get_state(&self) -> bool {true}

	fn update(self : Arc<Self>, n : usize) -> Result<(Arc<dyn State>, Option<bool>), &'static str> {
		if let Some(b) = self.mapping.get(n) {
			if *b {
				Ok((self, None))
			} else {
				Ok((self.rule.upgrade().expect("Do not drop the rule").off.read().unwrap().as_ref().unwrap().clone(), Some(false)))
			}
		} else {
			Err("no enough adjecents around")
		}
	}

	fn force_to(self : Arc<Self>, on : bool) -> (Arc<dyn State>, Option<bool>) {
		if on {
			(self, None)
		} else {
			(self.rule.upgrade().expect("Do not drop the rule").off.read().unwrap().as_ref().unwrap().clone(), Some(false))
		}
	}
}

impl State for Off {
	fn get_state(&self) -> bool {false}

	fn update(self : Arc<Self>, n : usize) -> Result<(Arc<dyn State>, Option<bool>), &'static str> {
		if let Some(b) = self.mapping.get(n) {
			if *b {
				Ok((self.rule.upgrade().expect("Do not drop the rule").on.read().unwrap().as_ref().unwrap().clone(), Some(true)))
			} else {
				Ok((self, None))
			}
		} else {
			Err("no enough adjecents around")
		}
	}

	fn force_to(self : Arc<Self>, on : bool) -> (Arc<dyn State>, Option<bool>) {
		if on {
			(self.rule.upgrade().expect("Do not drop the rule").on.read().unwrap().as_ref().unwrap().clone(), Some(true))
		} else {
			(self, None)
		}
	}
}

// ------------------------ rule ------------------------
use std::sync::RwLock;
pub struct Rule {
	on  : RwLock<Option<Arc<dyn State>>>,
	off : RwLock<Option<Arc<dyn State>>>,

}
impl Rule {
	pub fn from(vec_on : Vec<bool>, vec_off : Vec<bool>) -> Arc<Rule> {
		let rule = Rule {on : RwLock::new(None), off : RwLock::new(None)};
		let rc = Arc::new(rule);
		let on = Arc::new(On {
			rule : Arc::downgrade(&rc),
			mapping : vec_on
		}) as Arc<dyn State>;
		let off = Arc::new(Off {
			rule : Arc::downgrade(&rc),
			mapping : vec_off
		}) as Arc<dyn State>;
		rc.on.write().unwrap().replace(on);
		rc.off.write().unwrap().replace(off);
		rc
	}

	pub fn count(&self, on : bool) -> usize {
		if on {
			Arc::strong_count(&self.on.read().unwrap().as_ref().unwrap()) - 1
		} else {
			Arc::strong_count(&self.off.read().unwrap().as_ref().unwrap()) - 1
		}
	}

	pub fn on(self : &Arc<Self>) -> Option<Weak<dyn State>> {
		Some(Arc::downgrade(self.on.read().unwrap().as_ref().unwrap()))
	}

	pub fn off(self : &Arc<Self>) -> Option<Weak<dyn State>> {
		Some(Arc::downgrade(self.off.read().unwrap().as_ref().unwrap()))
	}
}
impl fmt::Display for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// self.on.read().unwrap().
		write!(f, "{}-{}", self.off.read().unwrap().as_ref().unwrap().as_ref(), self.on.read().unwrap().as_ref().unwrap().as_ref())
    }
}
impl fmt::Debug for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}::on \t: {}\n", self, self.count(false))?;
		// write!(f, "\n")?;
		write!(f, "{}::off\t: {}\n", self, self.count(true))
    }
}