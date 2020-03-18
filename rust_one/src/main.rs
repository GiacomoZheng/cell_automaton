#[allow(unused_imports)]
use rust_one::{Cell, On, Off, buildup_square_board, buildup_rect_board};

use std::rc::Rc;
use std::cell::RefCell;

// #[allow(unused_variables)]
fn main() {
	for temp_vec in buildup_square_board(4).iter() {
		for block in temp_vec.iter() {
			print!("{:?}\t", block.borrow());
		}
		println!();
		println!();
	}

	{
		// // !!!!!
		// let mut  on =  On::from(vec![false, false,  true,  true, false]);
		// let mut off = Off::from(vec![false,  true, false, false, false]);
		// let to_off = Rc::new(off); 
		// let  to_on = Rc::new( on);
		// to_on.turn_off = Some(RefCell::new(Rc::downgrade(&to_off)));
		// to_off.turn_on = Some(RefCell::new(Rc::downgrade( &to_on)));
		// // on
	};
}
