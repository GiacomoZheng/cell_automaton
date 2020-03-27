use super::{Cell, Config};

use std::rc::Rc;
use std::cell::RefCell;
#[allow(non_snake_case)]
// #[derive(Debug)]
pub struct Canvas {
	pub board : Vec<Vec<Option<Rc<RefCell<Cell>>>>>,
	pub m : usize,
	pub n : usize,
	pub Type : &'static str
}

impl Canvas {
	pub fn new(m : usize, n : usize) -> Canvas{
		Canvas {
			board : Vec::new(),
			m,
			n,
			Type : ""
		}
	}
}
use super::tools::Handle;
impl Canvas {
	pub fn init(& mut self, configs : Vec<Vec<Option<&Rc<Config>>>>) -> Result<(), &'static str> {
		for i in 0..self.n {
			for j in 0..self.m {
				if let Some(cell) = self.board.get(i).handle("uninitialized")?.get(j).handle("uninitialized")? {
					if let Some(config) = configs.get(i).handle("wrong initialization : need more configs")?
												.get(j).handle("wrong initialization : need more configs")? {
						cell.as_ref().borrow_mut().set(config);
				} else {
					return Err("wrong initialization : need a config");
					}
				}
			}
		}
		
		Ok(())
	}
}
impl Canvas {
	pub fn buildup_rect_board(& mut self) {
		self.Type = "rect";

		for i in 0..self.n {
			self.board.push(Vec::new());
			for _ in 0..self.m {
				self.board[i].push(Some(Rc::new(RefCell::new(Cell::new()))));
			}
		}

		// buildup the adjecents
		{
			let board = & mut self.board;
			let m = self.m;
			let n = self.n;
			board[0][0].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
				Rc::downgrade(&board[0+1][0  ].as_ref().unwrap()),
				Rc::downgrade(&board[0  ][0+1].as_ref().unwrap()),
				// Rc::downgrade(&board[0-1][0  ].as_ref().unwrap()),
				// Rc::downgrade(&board[0  ][0-1].as_ref().unwrap())
			]);
			for j in 1..m-1 {
				board[0][j].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
					Rc::downgrade(&board[0+1][j  ].as_ref().unwrap()),
					Rc::downgrade(&board[0  ][j+1].as_ref().unwrap()),
					// Rc::downgrade(&board[0-1][j  ].as_ref().unwrap()),
					Rc::downgrade(&board[0  ][j-1].as_ref().unwrap())
				]);
			}
			board[0][m-1].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
				Rc::downgrade(&board[0+1][m-1  ].as_ref().unwrap()),
				// Rc::downgrade(&board[0  ][m-1+1].as_ref().unwrap()),
				// Rc::downgrade(&board[0-1][m-1  ].as_ref().unwrap()),
				Rc::downgrade(&board[0  ][m-1-1].as_ref().unwrap())
			]);
			for i in 1..n-1 {
				board[i][0].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
					Rc::downgrade(&board[i+1][0  ].as_ref().unwrap()),
					Rc::downgrade(&board[i  ][0+1].as_ref().unwrap()),
					Rc::downgrade(&board[i-1][0  ].as_ref().unwrap()),
					// Rc::downgrade(&board[i  ][0-1].as_ref().unwrap())
				]);
				for j in 1..m-1 {
					board[i][j].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
						Rc::downgrade(&board[i+1][j  ].as_ref().unwrap()),
						Rc::downgrade(&board[i  ][j+1].as_ref().unwrap()),
						Rc::downgrade(&board[i-1][j  ].as_ref().unwrap()),
						Rc::downgrade(&board[i  ][j-1].as_ref().unwrap())
					]);
				}
				board[i][m-1].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
					Rc::downgrade(&board[i+1][m-1  ].as_ref().unwrap()),
					// Rc::downgrade(&board[i  ][m-1+1].as_ref().unwrap()),
					Rc::downgrade(&board[i-1][m-1  ].as_ref().unwrap()),
					Rc::downgrade(&board[i  ][m-1-1].as_ref().unwrap())
				]);
			}
			board[n-1][0].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
				// Rc::downgrade(&board[n-1+1][0  ].as_ref().unwrap()),
				Rc::downgrade(&board[n-1  ][0+1].as_ref().unwrap()),
				Rc::downgrade(&board[n-1-1][0  ].as_ref().unwrap()),
				// Rc::downgrade(&board[n-1  ][0-1].as_ref().unwrap())
			]);
			for j in 1..m-1 {
				board[n-1][j].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
					// Rc::downgrade(&board[n-1+1][j  ].as_ref().unwrap()),
					Rc::downgrade(&board[n-1  ][j+1].as_ref().unwrap()),
					Rc::downgrade(&board[n-1-1][j  ].as_ref().unwrap()),
					Rc::downgrade(&board[n-1  ][j-1].as_ref().unwrap())
				]);
			}
			board[n-1][m-1].as_ref().unwrap().borrow_mut().append_adj(&mut vec![
				// Rc::downgrade(&board[n-1+1][m-1  ].as_ref().unwrap()),
				// Rc::downgrade(&board[n-1  ][m-1+1].as_ref().unwrap()),
				Rc::downgrade(&board[n-1-1][m-1  ].as_ref().unwrap()),
				Rc::downgrade(&board[n-1  ][m-1-1].as_ref().unwrap())
			]);
		}
	}
}

use std::fmt;
impl fmt::Debug for Canvas {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..self.n {
			for j in 0..self.m {
				write!(f, "{}", match self.board[i][j].as_ref() { Some(cell) => format!("{:?}", cell.borrow()), None => String::from("\t")})?;
			}
			write!(f, "\n")?;
		}
		write!(f, "")
	
		// write!(f, "{:?}", "1")?;
		// write!(f, "{}", match self.board[0][0].as_ref() { Some(cell) => format!("{:?}", cell.borrow()), None => String::from("\t")})
    }
}