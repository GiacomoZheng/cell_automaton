use crate::{On, Off};
// use crate::cell::State;
use std::rc::{Rc, Weak};
// use std::cell::RefCell;
// use std::fmt;

pub struct Rule {
	on  : Option<Rc< On>>,
    off : Option<Rc<Off>>,
}

impl Rule { // ++++
    // pub fn new(state: RefCell<Rc< On>>) -> Rule {
	// 	Rule {
	// 		on  : Some(Rc::new(state.as_ref().into_inner())),
	// 		// off : state.turn_off.as_ref().map(|s| Rc::clone(&s.upgrade().unwrap())),
	// 		off : state.as_ref().borrow().turn_off.unwrap().borrow().upgrade()
	// 	}
	// }

	// + public static Rule from(List<Boolean> list_on, List<Boolean> list_off) {

	
}

// const B2_S23_on : Rc<On> = 

// pub struct _Rule;
// impl _Rule {
// 	fn  B2_S23(state : bool) ->  impl State {
// 		let mut  on =  On::from(vec![false, false,  true,  true, false]);
// 		let mut off = Off::from(vec![false,  true,  false,  false, false]);
// 		on.turn_off = Some(Rc::downgrade(&Rc::new(off)));
// 		off.turn_on = Some(Rc::downgrade(&Rc::new( on)));
// 		if state {
// 			on
// 		} else {
// 			off
// 		}
// 	}
