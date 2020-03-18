package machine.lib.cell;

import java.util.ArrayList;
import java.util.List;

public class Off implements State {
	// rule : Vec<bool>,
	List<Boolean> rule;
	// pub turn_on : Option<RefCell<Weak<On>>>
	public On turn_on;

	// pub fn new() -> Off {
	public static Off init() {
		return from(new ArrayList<Boolean>());
	}

	// pub fn from(v : Vec<bool>) -> Off {
	public static Off from(List<Boolean> v) {
		Off off = new Off();
		off.rule = v;
		off.turn_on = null;
		return off;
	}

// impl State for Off {
	// fn get_state(&self) -> bool {false}
	public Boolean get_state() {return false;} // ! public

	// fn update(self : Rc<Self>, n : usize) -> Result<Rc<dyn State>, &'static str> {
	public State update(int n) throws Exception {
		if (rule.get(n)) {
			return turn_on;
		} else {
			return this;
		}
	}
}