package machine;

import machine.lib.tools.Canvas;
import machine.lib.Rule;
import machine.lib.Config;
import machine.lib.screen.ScreenModule;

import java.util.Arrays;
import java.util.List;

import java.awt.EventQueue;


public class Main {
	public static void main(String[] args) throws Exception {
		Config sn =  Rule.B2_S23_4.on();
		Config sf = Rule.B2_S23_4.off();
		Config ln =  Rule.toLIVE_4.on();
		Config lf = Rule.toDEAD_4.off();

		Config pn = Rule.toDEAD_4.on(); // pulse_on
		Config pf = Rule.toLIVE_4.on(); // pulse_off


		// Canvas canvas = new Canvas(6, 6);
		// canvas.buildup_rect_board();
		// List<List<Config>> configs = Arrays.asList(
		// 	Arrays.asList( sf, sf, sf, sf, sf, sf ),
		// 	Arrays.asList( sf, sf, sf, sf, sf, sf ),
		// 	Arrays.asList( sf, sf, sf, sf, sf, sf ),
		// 	Arrays.asList( sf, sf, sf, sf, sf, sf ),
		// 	Arrays.asList( sf, sf, sf, sf, sf, sf ),
		// 	Arrays.asList( sf, sf, sf, sf, sf, sf )
		// );

		Rule B1 = Rule.from(Arrays.asList( false, false, false, false, false ), Arrays.asList( false,  true,  false, false, true ));
		Config n1 = B1.on();
		Config f1 = B1.off();
		Rule B2 = Rule.from(Arrays.asList( false, false, false, false, false ), Arrays.asList( false,  false,  true, false, false ));
		Config n2 = B2.on();
		Config f2 = B2.off();

		Canvas canvas = new Canvas(10, 3);
		canvas.buildup_rect_board();
		List<List<Config>> configs = Arrays.asList(
			Arrays.asList(  lf,  f1,  f2,  f2,  f2,  f2,  f2,  f2,  f2,  f2 ),
			Arrays.asList(  ln,  f1,  f1,  f1,  f1,  f1,  f1,  f1,  f1,  f1 ),
			Arrays.asList(  lf,  f1,  f2,  f2,  f2,  f2,  f2,  f2,  f2,  f2 )
		);
		canvas.init(configs);

		// command line test
		// boolean debug_config = false;
		// canvas.debug(debug_config);
		// while (true) {
		// 	canvas.update();
			
		// 	System.in.read();

		// 	canvas.debug(debug_config);
		// }

		// gui test
		EventQueue.invokeLater(new Runnable() {
			@Override
			public void run() {
				try {
					ScreenModule frame = new ScreenModule(canvas);
					frame.setVisible(true);
					
				} catch (Exception e) {
					e.printStackTrace();
				}
			}
		});
	}
	
}