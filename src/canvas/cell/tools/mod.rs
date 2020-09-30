pub enum BoardType {
	Rect, // ? add Rect_Screen and Rect_Assistant_Sreen

	UnInitialized
}
use BoardType::*;

/// Layout(board_type : BoardType, m : usize, n : usize, height : usize)
pub struct Layout {
	board_type : BoardType,
	m : usize, // x_bounded
	n : usize, // y_bounded
	height : usize
}

impl Layout {
	pub fn set_type(&mut self, board_type : BoardType) {
		self.board_type = board_type;
	}

	pub  fn get_type(&self) -> &BoardType {
		&self.board_type
	}
}

impl Layout {
	fn get_area(&self) -> usize {
		self.m * self.n
	}

	pub fn get_height(&self) -> usize {
		self.height
	}

	pub fn get_len(&self) -> usize {
		self.get_area() * self.get_height()
	}

	pub fn x_bound(&self) -> usize {
		self.m
	}
	pub fn y_bound(&self) -> usize {
		self.n
	}

	pub fn new(m : usize, n : usize, height : usize) -> Layout {
		Layout {
			board_type : UnInitialized,
			m,
			n,
			height
		}
	}
}

#[allow(dead_code)]
impl Layout { // basic for index
	pub fn index_of(&self, x : usize, y : usize, z : usize) -> usize {x + y * self.m + z * self.get_area()}
	pub fn sceen_index_of(&self, x : usize, y : usize) -> usize {x + y * self.m + (self.get_height() - 1) * self.get_area()}

	fn above_one(&self, index : usize) -> usize {index - self.m}
	fn below_one(&self, index : usize) -> usize {index + self.m}
	#[allow(non_snake_case)]
	fn left__one(&self, index : usize) -> usize {index -      1}
	fn right_one(&self, index : usize) -> usize {index +      1}

	fn la_one(&self, index : usize) -> usize {index - self.m - 1}
	fn lb_one(&self, index : usize) -> usize {index + self.m - 1}
	fn ra_one(&self, index : usize) -> usize {index - self.m + 1}
	fn rb_one(&self, index : usize) -> usize {index + self.m + 1}

	fn higher_one(&self, index : usize) -> usize {index + self.get_area()}
	#[allow(non_snake_case)]
	fn lower__one(&self, index : usize) -> usize {index - self.get_area()}
}

impl Layout { // for different types
	/// input a index and output all the adjecents' indexes
	pub fn adjecents(&self, index : usize) -> Vec<usize> {
		let x = index % self.m; // m
		let y = (index / self.m) % self.get_area();
		let z = index / self.get_area();

		match self.board_type {
			Rect => {
				if z == 0 {
					if x == 0 {
						if y == 0 {
							vec![self.higher_one(index), self.right_one(index), self.below_one(index)]
						} else if y < self.n - 1 {
							vec![self.higher_one(index), self.right_one(index), self.below_one(index), self.above_one(index)]
						} else {
							vec![self.higher_one(index), self.right_one(index), self.above_one(index)]
						}
					} else if x < self.m - 1 {
						if y == 0 {
							vec![self.higher_one(index), self.right_one(index), self.below_one(index), self.left__one(index)]
						} else if y < self.n - 1 {
							vec![self.higher_one(index), self.right_one(index), self.below_one(index), self.above_one(index), self.left__one(index)]
						} else {
							vec![self.higher_one(index), self.right_one(index), self.above_one(index), self.left__one(index)]
						}
					} else {
						if y == 0 {
							vec![self.higher_one(index), self.below_one(index), self.left__one(index)]
						} else if y < self.n - 1 {
							vec![self.higher_one(index), self.below_one(index), self.above_one(index), self.left__one(index)]
						} else {
							vec![self.higher_one(index), self.above_one(index), self.left__one(index)]
						}
					}
				} else if z < self.get_height() - 1 {
					if x == 0 {
						if y == 0 {
							vec![self.lower__one(index), self.higher_one(index), self.right_one(index), self.below_one(index)]
						} else if y < self.n - 1 {
							vec![self.lower__one(index), self.higher_one(index), self.right_one(index), self.below_one(index), self.above_one(index)]
						} else {
							vec![self.lower__one(index), self.higher_one(index), self.right_one(index), self.above_one(index)]
						}
					} else if x < self.m - 1 {
						if y == 0 {
							vec![self.lower__one(index), self.higher_one(index), self.right_one(index), self.below_one(index), self.left__one(index)]
						} else if y < self.n - 1 {
							vec![self.lower__one(index), self.higher_one(index), self.right_one(index), self.below_one(index), self.above_one(index), self.left__one(index)]
						} else {
							vec![self.lower__one(index), self.higher_one(index), self.right_one(index), self.above_one(index), self.left__one(index)]
						}
					} else {
						if y == 0 {
							vec![self.lower__one(index), self.higher_one(index), self.below_one(index), self.left__one(index)]
						} else if y < self.n - 1 {
							vec![self.lower__one(index), self.higher_one(index), self.below_one(index), self.above_one(index), self.left__one(index)]
						} else {
							vec![self.lower__one(index), self.higher_one(index), self.above_one(index), self.left__one(index)]
						}
					}
				} else {
					vec![self.lower__one(index)]
				}
			},
			UnInitialized => panic!("undefined right now")
		}
	}
}

#[test]
fn adj() {
	let mut layout = Layout::new(3, 3, 2);
	layout.set_type(Rect);
	for index in 0..layout.get_len() {
		eprintln!("{}:{:?}", index, layout.adjecents(index));
		for item in layout.adjecents(index) {
			eprintln!("{} < {}", item, layout.get_len());
			assert!(item < layout.get_len());
		}
	}
}