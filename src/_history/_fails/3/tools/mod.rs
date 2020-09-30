use std::rc::Rc;
use std::cell::RefCell;

use crate::cell::Cell;

pub fn buildup_rect_board(n : usize, m : usize) -> Vec<Vec<Rc<RefCell<Cell>>>> {
	let mut board : Vec<Vec<Rc<RefCell<Cell>>>> = Vec::new();

	for i in 0..n {
		board.push(Vec::new());
		for _ in 0..m {
			board[i].push(Rc::new(RefCell::new(Cell::default())));
		}
	}
	
	// initialize
	{
		board[0][0].borrow_mut().append_adj(&mut vec![
			Rc::downgrade(&board[0+1][0  ]),
			Rc::downgrade(&board[0  ][0+1]),
			// Rc::downgrade(&board[0-1][0  ]),
			// Rc::downgrade(&board[0  ][0-1])
		]);
		for j in 1..m-1 {
			board[0][j].borrow_mut().append_adj(&mut vec![
				Rc::downgrade(&board[0+1][j  ]),
				Rc::downgrade(&board[0  ][j+1]),
				// Rc::downgrade(&board[0-1][j  ]),
				Rc::downgrade(&board[0  ][j-1])
			]);
		}
		board[0][m-1].borrow_mut().append_adj(&mut vec![
			Rc::downgrade(&board[0+1][m-1  ]),
			// Rc::downgrade(&board[0  ][m-1+1]),
			// Rc::downgrade(&board[0-1][m-1  ]),
			Rc::downgrade(&board[0  ][m-1-1])
		]);
		for i in 1..n-1 {
			board[i][0].borrow_mut().append_adj(&mut vec![
				Rc::downgrade(&board[i+1][0  ]),
				Rc::downgrade(&board[i  ][0+1]),
				Rc::downgrade(&board[i-1][0  ]),
				// Rc::downgrade(&board[i  ][0-1])
			]);
			for j in 1..m-1 {
				board[i][j].borrow_mut().append_adj(&mut vec![
					Rc::downgrade(&board[i+1][j  ]),
					Rc::downgrade(&board[i  ][j+1]),
					Rc::downgrade(&board[i-1][j  ]),
					Rc::downgrade(&board[i  ][j-1])
				]);
			}
			board[i][m-1].borrow_mut().append_adj(&mut vec![
				Rc::downgrade(&board[i+1][m-1  ]),
				// Rc::downgrade(&board[i  ][m-1+1]),
				Rc::downgrade(&board[i-1][m-1  ]),
				Rc::downgrade(&board[i  ][m-1-1])
			]);
		}
		board[n-1][0].borrow_mut().append_adj(&mut vec![
			// Rc::downgrade(&board[n-1+1][0  ]),
			Rc::downgrade(&board[n-1  ][0+1]),
			Rc::downgrade(&board[n-1-1][0  ]),
			// Rc::downgrade(&board[n-1  ][0-1])
		]);
		for j in 1..m-1 {
			board[n-1][j].borrow_mut().append_adj(&mut vec![
				// Rc::downgrade(&board[n-1+1][j  ]),
				Rc::downgrade(&board[n-1  ][j+1]),
				Rc::downgrade(&board[n-1-1][j  ]),
				Rc::downgrade(&board[n-1  ][j-1])
			]);
		}
		board[n-1][m-1].borrow_mut().append_adj(&mut vec![
			// Rc::downgrade(&board[n-1+1][m-1  ]),
			// Rc::downgrade(&board[n-1  ][m-1+1]),
			Rc::downgrade(&board[n-1-1][m-1  ]),
			Rc::downgrade(&board[n-1  ][m-1-1])
		]);
	}

	board
}

pub fn buildup_square_board(n : usize) -> Vec<Vec<Rc<RefCell<Cell>>>> {
	buildup_rect_board(n, n)
}