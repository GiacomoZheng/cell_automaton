// !!!!
package machine.test;

import machine.lib.tools.Canvas;
import machine.lib.screen.ScreenModule;
import static machine.test.Def.*;

import java.util.Arrays;

import java.awt.EventQueue;

public class Diode {
	// ! if there is a flow
	public static void main(String[] args) throws Exception {
		Canvas canvas = new Canvas(12, 3);
		canvas.buildup_rect_board();
		canvas.init(Arrays.asList(
			Arrays.asList(  lf,  lf, f13,  lf, n13,  lf, f13,  lf, f13,  lf, f13,  lf ),
			Arrays.asList(  sn, f13, f13, f23, n02, n23, f02, f13, f13, f13, f13,  sf ),
			Arrays.asList(  lf, f13,  lf, f13,  lf, n13,  lf, f13,  lf, f13,  lf,  lf )
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