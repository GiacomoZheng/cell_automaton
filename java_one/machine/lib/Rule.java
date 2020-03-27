package machine.lib;

import machine.lib.cell.On;

import java.util.Arrays;
import java.util.List;

import machine.lib.cell.Off;

public class Rule {
	public  On  on;
	public Off off;

	public static final Rule B2_S23_4 = Rule.from(Arrays.asList(false, false,  true,  true, false), Arrays.asList(false, false,  true, false, false));
	public static final Rule toDEAD_4 = Rule.from(Arrays.asList(false, false, false, false, false), Arrays.asList(false, false, false, false, false));
	public static final Rule toLIVE_4 = Rule.from(Arrays.asList( true,  true,  true,  true,  true), Arrays.asList( true,  true,  true,  true,  true));


	Rule(On state) { // ! deprecated on rust
		// Rule rule = Rule();
		this.on  = state;
		this.off = state.turn_off;
	}

	Rule(Off state) {
		// Rule rule = Rule();
		this.off  = state;
		this.on = state.turn_on;
	}

	public static Rule from(List<Boolean> list_on, List<Boolean> list_off) {
		On   on =  On.from( list_on);
		Off off = Off.from(list_off);
		on.turn_off = off;
		off.turn_on =  on;
		return new Rule(on);
	}

	public Config on() {
		return new Config( true, this);
	}
	public Config off() {
		return new Config(false, this);
	}

	public static void main(String[] args) {
		
	}
}