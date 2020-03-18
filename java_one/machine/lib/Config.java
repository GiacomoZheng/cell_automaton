package machine.lib;

import machine.lib.cell.On;
import machine.lib.cell.Off;

public class Config {
	public Boolean on;
	public Rule rule;

	Config (Boolean on, Rule rule) {
		this.on = on;
		this.rule = rule;
	}
	Config (On state) {
		this(true, new Rule(state));
	}
	Config (Off state) {
		this(false, new Rule(state));
	}

}