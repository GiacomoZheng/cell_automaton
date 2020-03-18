package machine.lib.cell;

import java.util.ArrayList;
import java.util.List;

public class On implements State {
	// rule : Vec<bool>,
	private List<Boolean> rule;
	// pub turn_off : Option<RefCell<Weak<On>>>
	public Off turn_off;

	// pub fn new() -> On {
	public static On init() {
		return from(new ArrayList<Boolean>());
	}

	// pub fn from(v : Vec<bool>) -> On {
	public static On from(List<Boolean> v) {
		On on = new On();
		on.rule = v;
		on.turn_off = null;
		return on;
	}

// impl State for On {
	// fn get_state(&self) -> bool {true}
	public Boolean get_state() {return true;} // ! public

	// fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str> {
	public State update(int n) throws Exception {
		if (rule.get(n)) {
			return this;
		} else {
			return turn_off;
		}
	}
}