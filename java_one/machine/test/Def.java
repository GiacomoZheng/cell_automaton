package machine.test;

import machine.lib.Rule;
import machine.lib.Config;

import java.util.Arrays;

public class Def {
	public final static Config ln =  Rule.toLIVE_4.on();
	public final static Config lf = Rule.toDEAD_4.off();

	public final static Config pn = Rule.toDEAD_4.on(); // pulse_on
	public final static Config pf = Rule.toLIVE_4.on(); // pulse_off

	public final static Rule B13S13 = Rule.from(Arrays.asList( false,  true,  false,  true ), Arrays.asList( false,  true,  false,  true ));
	public final static Config n13 = B13S13.on();
	public final static Config f13 = B13S13.off();
	public final static Rule B2S2 = Rule.from(Arrays.asList( false, false,  true, false ), Arrays.asList( false,  false,  true, false ));
	public final static Config n2 = B2S2.on();
	public final static Config f2 = B2S2.off();

	public final static Rule Bg1 = Rule.from(Arrays.asList( false,  true,  true,  true ), Arrays.asList( false,  true,  true,  true ));
	public final static Config ng1 = Bg1.on();
	public final static Config fg1 = Bg1.off();
	
	public final static Rule Self = Rule.from(Arrays.asList(  true,  true,  true,  true,  true ), Arrays.asList( false,  false, false, false, false ));
	public final static Config sn = Self.on();
	public final static Config sf = Self.off();
	
}