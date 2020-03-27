// !!!
package machine.test;

import machine.lib.tools.Canvas;
import machine.lib.screen.ScreenModule;
import static machine.test.Def.*;

import java.util.Arrays;

import java.awt.EventQueue;

public class Cross {
	public static void main(String[] args) throws Exception {
		Canvas canvas = new Canvas(7, 7);
		canvas.buildup_rect_board();
		canvas.init(Arrays.asList(
			// + 
			Arrays.asList(  lf,  lf,  lf,  lf,  lf,  lf,  lf ),
			Arrays.asList(  lf,  lf, f13,  lf,  ln,  lf,  lf ),
			Arrays.asList(  lf, f13,  lf,  lf,  f2,  lf,  lf ),
			Arrays.asList(  sn, f13, f13, f13, f13, f13, f13 ),
			Arrays.asList(  lf,  lf,  f2,  lf,  lf, f13,  lf ),
			Arrays.asList(  lf,  lf,  ln,  lf, f13,  lf,  lf ),
			Arrays.asList(  lf,  lf,  lf,  lf,  lf,  lf,  lf )
		));

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