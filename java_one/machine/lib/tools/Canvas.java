package machine.lib.tools;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import machine.lib.cell.Cell;
import machine.lib.Config;

public class Canvas {
	Canvas () throws Exception {
		throw new Exception();
	}
	
	public List<List<Cell>> board;
	public int m;
	public int n;
	public String type;

	public Canvas(int length, int width) {
		board = new ArrayList<List<Cell>>();
		m = length;
		n = width;
	};

	/** use it after buildup */
	public void init(List<List<Config>> configs) throws Exception {
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < m; j++) {
				if (board.get(i).get(j) != null) {
					// System.out.println(i + ", " + j);
					board.get(i).get(j).set(configs.get(i).get(j)); // + if wrong here add some infor
				}
			}
		}
	}

	public void refresh() {
		// make sure all the count's has been stored
		for (List<Cell> temp_list : board) {
			for (Cell cell : temp_list) {
				cell.count();
			}
		}
	}

	public void update() throws Exception {
		refresh();
		for (List<Cell> temp_list : board) {
			for (Cell cell : temp_list) {
				// + if it is long-some (longlive or longdead), it can be skipped
				cell.update();
			}
		}
	}

	// ------------------------------
	public void buildup_tri_board() {
		type = "tri";
		List<Cell> even_line = new ArrayList<Cell>();
		List<Cell>  odd_line = new ArrayList<Cell>();

		for (int j = 0; j < m - 3; j += 4) {
			even_line.add(Cell.longdead());
			even_line.add(null);
			even_line.add(null);
			even_line.add(Cell.longdead());
			
			odd_line.add(null);
			odd_line.add(Cell.longdead());
			odd_line.add(Cell.longdead());
			odd_line.add(null);
		}
		// odd_line .addAll(new ArrayList<Cell>( odd_line.subList(0, m % 4)));
		for (Cell cell : odd_line.subList(0, m % 4)) {
			if (cell != null) {
				odd_line.add(Cell.longdead());
			} else {
				odd_line.add(null);
			}
		}
		// even_line.addAll(new ArrayList<Cell>(even_line.subList(0, m % 4)));
		for (Cell cell : even_line.subList(0, m % 4)) {
			if (cell != null) {
				even_line.add(Cell.longdead());
			} else {
				even_line.add(null);
			}
		}

		for (int i = 0; i < n - 1; i += 2) {
			board.add(new ArrayList<Cell>());
			for (Cell cell : even_line) {
				if (cell != null) {
					board.get(i).add(Cell.longdead());
				} else {
					board.get(i).add(null);
				}
			}
			board.add(new ArrayList<Cell>());
			for (Cell cell : odd_line) {
				if (cell != null) {
					board.get(i+1).add(Cell.longdead());
				} else {
					board.get(i+1).add(null);
				}
			}
		}
		// board.addAll(new ArrayList<List<Cell>>( board.subList(0, n % 2)));
		for (List<Cell> list : board.subList(0, n % 2)) {
			List<Cell> last_line = new ArrayList<Cell>();
			for (Cell cell : list) {
				if (cell != null) {
					last_line.add(Cell.longdead());
				} else {
					last_line.add(null);
				}
			}
			board.add(last_line);
		}

		{ // transport form
		// for (int j = 0; j < m-1; j += 2) {
		// 	even_line.add(Cell.longdead());
		// 	even_line.add(null);
		// 	odd_line.add(null);
		// 	odd_line.add(Cell.longdead());
		// }
		// if (m % 2 == 1) {
		// 	even_line.add(Cell.longdead());
		// 	odd_line.add(null);
		// 	// even_line.add(null);
		// 	// odd_line.add(Cell.longdead());
		// }

		// for (int i = 0; i < n - 1; i += 4) {
		// 	board.add(new ArrayList<Cell>(even_line));
		// 	board.add(new ArrayList<Cell>( odd_line));
		// 	board.add(new ArrayList<Cell>( odd_line));
		// 	board.add(new ArrayList<Cell>(even_line));
		// }
		}

		{
		// initialize
		Cell tempCell;
		tempCell = board.get(0).get(0);
		tempCell.append_adj(Arrays.asList(
			// (board.get(0-1).get(0+1)),
			(board.get(0+1).get(0+1))
			// (board.get(0  ).get(0-1))
		));
		for (int j = 1; j < m-1; j++) {
			tempCell = board.get(0).get(j);
			if (tempCell != null) {
				if (board.get(0).get(j-1) == null) {
					tempCell.append_adj(Arrays.asList(
						// (board.get(0-1).get(j-1)),
						(board.get(0+1).get(j-1)),
						(board.get(0  ).get(j+1))
					));
				} else {
					tempCell.append_adj(Arrays.asList(
						// (board.get(0-1).get(j+1)),
						(board.get(0+1).get(j+1)),
						(board.get(0  ).get(j-1))
					));
				}
			}
		}
		tempCell = board.get(0).get(m-1);
		if (tempCell != null) {
			if (board.get(0).get(m-1-1) == null) {
				tempCell.append_adj(Arrays.asList(
					// (board.get(0-1).get(m-1-1)),
					(board.get(0+1).get(m-1-1))
					// (board.get(0  ).get(m-1+1))
				));
			} else {
				tempCell.append_adj(Arrays.asList(
					// (board.get(0-1).get(m-1+1)),
					// (board.get(0+1).get(m-1+1)),
					(board.get(0  ).get(m-1-1))
				));
			}
		}
		for (int i = 1; i < n-1; i++) {
			tempCell = board.get(i).get(0);
			if (tempCell != null) {
				tempCell.append_adj(Arrays.asList(
					(board.get(i-1).get(0+1)),
					(board.get(i+1).get(0+1))
					// (board.get(i  ).get(0-1))
				));
			}

			for (int j = 1; j < m-1; j++) {
				tempCell = board.get(i).get(j);
				if (tempCell != null) {
					if (board.get(i).get(j-1) == null) {
						tempCell.append_adj(Arrays.asList(
							(board.get(i-1).get(j-1)),
							(board.get(i+1).get(j-1)),
							(board.get(i  ).get(j+1))
						));
					} else {
						tempCell.append_adj(Arrays.asList(
							(board.get(i-1).get(j+1)),
							(board.get(i+1).get(j+1)),
							(board.get(i  ).get(j-1))
						));
					}
				}
			}
			
			tempCell = board.get(i).get(m-1);
			if (tempCell != null) {
				if (board.get(i).get(m-1-1) == null) {
					tempCell.append_adj(Arrays.asList(
						(board.get(i-1).get(m-1-1)),
						(board.get(i+1).get(m-1-1))
						// (board.get(i  ).get(m-1+1))
					));
				} else {
					tempCell.append_adj(Arrays.asList(
						// (board.get(i-1).get(m-1+1)),
						// (board.get(i+1).get(m-1+1)),
						(board.get(i  ).get(m-1-1))
					));
				}
			}
		}
		tempCell = board.get(n-1).get(0);
		if (tempCell != null) {
			tempCell.append_adj(Arrays.asList(
				(board.get(n-1-1).get(0+1))
				// (board.get(n-1+1).get(0+1)),
				// (board.get(n-1  ).get(0-1))
			));
		}
		for (int j = 1; j < m-1; j++) {
			tempCell = board.get(n-1).get(j);
			if (tempCell != null) {
				if (board.get(n-1).get(j-1) == null) {
					tempCell.append_adj(Arrays.asList(
						(board.get(n-1-1).get(j-1)),
						// (board.get(n-1+1).get(j-1)),
						(board.get(n-1  ).get(j+1))
					));
				} else {
					tempCell.append_adj(Arrays.asList(
						(board.get(n-1-1).get(j+1)),
						// (board.get(n-1+1).get(j+1)),
						(board.get(n-1  ).get(j-1))
					));
				}
			}
		}
		tempCell = board.get(n-1).get(m-1);
		if (tempCell != null) {
			if (board.get(n-1).get(m-1-1) == null) {
				tempCell.append_adj(Arrays.asList(
					(board.get(n-1-1).get(m-1-1))
					// (board.get(n-1+1).get(m-1-1))
					// (board.get(n-1  ).get(m-1+1))
				));
			} else {
				tempCell.append_adj(Arrays.asList(
					// (board.get(n-1-1).get(m-1+1)),
					// (board.get(n-1+1).get(m-1+1)),
					(board.get(n-1  ).get(m-1-1))
				));
			}
		} 
		
		} // end initialize

	}

	public void buildup_rect_board() {
		type = "rect";
		for (int i = 0; i < n; i++) {
			board.add(new ArrayList<Cell>());
			for (int j = 0; j < m; j++) {
				board.get(i).add(Cell.longdead());
			}
		}

		// initialize
		board.get(0).get(0).append_adj(Arrays.asList(
			(board.get(0+1).get(0  )),
			(board.get(0  ).get(0+1))
			// Rc::downgrade(board[0-1].get(0  )),
			// Rc::downgrade(board.get(0  )[0-1])
		));
		for (int j = 1; j < m-1; j++) {
			board.get(0).get(j).append_adj(Arrays.asList(
				(board.get(0+1).get(j  )),
				(board.get(0  ).get(j+1)),
				// Rc::downgrade(board[0-1].get(j  )),
				(board.get(0  ).get(j-1))
			));
		}
		board.get(0).get(m-1).append_adj(Arrays.asList(
			(board.get(0+1).get(m-1  )),
			// Rc::downgrade(board.get(0  )[m-1+1]),
			// Rc::downgrade(board[0-1].get(m-1  )),
			(board.get(0  ).get(m-1-1))
		));
		for (int i = 1; i < n-1; i++) {
			board.get(i).get(0).append_adj(Arrays.asList(
				(board.get(i+1).get(0  )),
				(board.get(i  ).get(0+1)),
				(board.get(i-1).get(0  ))
				// Rc::downgrade(board.get(i  ).get(0-1))
			));
			for (int j = 1; j < m-1; j++) {
				board.get(i).get(j).append_adj(Arrays.asList(
					(board.get(i+1).get(j  )),
					(board.get(i  ).get(j+1)),
					(board.get(i-1).get(j  )),
					(board.get(i  ).get(j-1))
				));
			}
			board.get(i).get(m-1).append_adj(Arrays.asList(
				(board.get(i+1).get(m-1  )),
				// Rc::downgrade(board[i  ][m-1+1]),
				(board.get(i-1).get(m-1  )),
				(board.get(i  ).get(m-1-1))
			));
		}
		board.get(n-1).get(0).append_adj(Arrays.asList(
			// Rc::downgrade(board[n-1+1].get(0  )),
			(board.get(n-1  ).get(0+1)),
			(board.get(n-1-1).get(0  ))
			// Rc::downgrade(board.get(n-1  )[0-1])
		));
		for (int j = 1; j < m-1; j++) {
			board.get(n-1).get(j).append_adj(Arrays.asList(
				// Rc::downgrade(board[n-1+1].get(j  )),
				(board.get(n-1  ).get(j+1)),
				(board.get(n-1-1).get(j  )),
				(board.get(n-1  ).get(j-1))
			));
		}
		board.get(n-1).get(m-1).append_adj(Arrays.asList(
			// Rc::downgrade(board[n-1+1].get(m-1  )),
			// Rc::downgrade(board.get(n-1  )[m-1+1]),
			(board.get(n-1-1).get(m-1  )),
			(board.get(n-1  ).get(m-1-1))
		));
	}


	// -----------------------------------
	public String toString() {
		StringBuilder string = new StringBuilder();
		switch(type) {
			case "rect":
				for (int i = 0; i < n-1; i++) {
					for (int j = 0; j < m-1; j++) {
						string.append(board.get(i).get(j) + " ---\t");
					}
					string.append(board.get(i).get(m-1) + "\n");
					for (int j = 0; j < m-1; j++) {
						string.append(" |  " + (board.get(i).get(j).count_adjs()) + "\t");
					}
					string.append(" |  " + (board.get(i).get(m-1).count_adjs()) + "\n");

					for (int j = 0; j < m-1; j++) {
						string.append(" |\t");
					}
					string.append(" |\n");
				}
				for (int j = 0; j < m-1; j++) {
					string.append(board.get(n-1).get(j) + " ---\t");
				}
				string.append(board.get(n-1).get(m-1) + "\n");
				for (int j = 0; j < m-1; j++) {
					string.append("    " + (board.get(n-1).get(j).count_adjs()) + "\t");
				}
				string.append("    " + (board.get(n-1).get(m-1).count_adjs()) + "\n");
				break;
			// case "tri":

		}
		return string.toString();
	}

	public void debug(boolean compact) {
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < m; j++) {
				System.out.print(board.get(i).get(j).debug(compact));
			}
			System.out.print("\n");
			if (!compact) { System.out.print("\n\n"); }
		}
	}
	public void debug() {
		debug(false);
	}

	public static void main(String[] args) {
		Canvas canvas = new Canvas(4, 3);
		canvas.buildup_rect_board();
		// System.out.println(canvas.board);//.get(0));//.get(4));
		// System.out.println(canvas);
		canvas.debug();
	}
	
}
