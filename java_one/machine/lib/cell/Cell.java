package machine.lib.cell;

import machine.lib.Config;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Cell {
	private Cell() {};
	// state: Option<Rc<dyn State>>,
	private State state;
	// adjecents: Vec<Weak<RefCell<Cell>>>
	private List<Cell> adjecents;

	private Integer count;

	public void set(Config config) throws Exception {
		if (config == null) {throw new Exception("wrong initialization");}
		if (config.on) {
			state = config.rule.on;
		} else {
			state = config.rule.off;
		}
	}

	// fn get_state(&self) -> bool {
	public Boolean get_state() {
		return state.get_state();
	}

	public int count() { // make it into a closure
		if (count != null) {return count; }
		int counter = 0;
		for (Cell cell : adjecents) {
			if (cell.get_state()) {
				counter++;
			}
		}
		count = counter;
		return counter;
	}

	public int count_adjs() { // for debug
		return adjecents.size();
	}

	private void clear() {
		count = null;
	}
	public void update() throws Exception {
		state = state.update(count()); // * update
		clear();
	}

	private void clear_adj() {
		for (Cell cell: adjecents) {
			cell.count = null;
		}
	}
	public void update(boolean on) {
		// only for clicked button

		state = state.update(on);
		clear_adj();
	}

	public void append_adj(List<Cell> v) {
		adjecents.addAll(v);
	}

	// -----------------------------------
	public static Cell longlive() {
		Cell cell = new Cell();
		cell.state =  On.from(Arrays.asList( true,  true,  true,  true,  true));
		cell.adjecents = new ArrayList<Cell>();
		return cell;
	}

	public static Cell longdead() {
		Cell cell = new Cell();
		cell.state = Off.from(Arrays.asList(false, false, false, false, false));
		cell.adjecents = new ArrayList<Cell>();
		return cell;
	}

	public String toString() {
		if (get_state()) {
			return "[■]";
		} else {
			return "[□]";
		}
	}
	
	public String debug(boolean compact) {
		String center;
		if (get_state()) {
			center = String.valueOf("[■]");
		} else {
			center = String.valueOf("[□]");
		}
		if (compact) {
			return center;
		} else {
			return String.format("%s:%s\t", center, count());
		}
	}
	public String debug() {
		return debug(false);
	}
}

interface State {
	// fn get_state(&self) -> bool;
	Boolean get_state();
	// fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str>;
	State update(int n) throws Exception;

	State update(boolean on);
}