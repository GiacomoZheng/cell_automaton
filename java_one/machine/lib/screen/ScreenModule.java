package machine.lib.screen;

import java.awt.GridLayout;
import java.awt.FlowLayout;

import javax.swing.Box;
import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JPanel;

import machine.lib.cell.Cell;
import machine.lib.tools.Canvas;

import java.util.ArrayList;
import java.util.List;

import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;

import java.awt.Color;

public class ScreenModule extends JFrame {
	private static final long serialVersionUID = 8617456729265392099L;

	private Canvas canvas;

	private JPanel mainPanel;
	private JPanel meau;
	private Box vBox;
	private List<List<CellBlock>> board;

	/**
	 * Create the frame.
	 */
	public ScreenModule(Canvas canvas) {
		this.canvas = canvas;
		int xnum = canvas.m;
		int ynum = canvas.n;
		int clen = 50;

		mainPanel = new JPanel(new GridLayout(canvas.n, canvas.m));
		mainPanel.setSize(xnum * clen, ynum * clen);
		// mainPanel.setSize();

		board = new ArrayList<List<CellBlock>>();
		for (List<Cell> list : canvas.board) {
			ArrayList<CellBlock> blockList = new ArrayList<CellBlock>();
			for (Cell cell : list) {
				CellBlock btn = new CellBlock(cell);
				btn.setText(String.valueOf(cell.count()));
				if (cell.get_state()) {
					// + make it clicked initially
					btn.on();
				} else {
					// + make it released initially
					btn.off();
				}
				// btn.setSize(clen, clen);
				mainPanel.add(btn);
				blockList.add(btn);
			}
			board.add(blockList);
		}

		meau = new JPanel(new FlowLayout(FlowLayout.CENTER));
		JButton btnRefresh =  new JButton("refresh");
		btnRefresh.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent arg0) {
				// System.out.println(cell.get_state());
				refresh();
			}
		});
		JButton btnUpdate =  new JButton("update");
		btnUpdate.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent arg0) {
				// System.out.println(cell.get_state());
				try {
					update();
				} catch (Exception e) {
					System.out.println("error in update");
					System.exit(0);
				}
			}
		});

		meau.add(btnRefresh); // +
		meau.add(btnUpdate);
		meau.setSize(xnum * clen, 50);

		vBox = Box.createVerticalBox();
		vBox.add(mainPanel);
		vBox.add(meau);

		// --- 
		setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

		// setAlwaysOnTop(true);
		// setResizable(false);
		
		setSize(mainPanel.getWidth(), mainPanel.getHeight() + meau.getHeight());
		setLocation(100, 100);
		
		setContentPane(vBox);
		// pack();
	}

	private void refresh() {
		System.out.println("refresh");
		canvas.refresh();
		for (List<CellBlock> list : board) {
			for (CellBlock btn : list) {
				btn.setText(String.valueOf(btn.cell.count()));
			}
		}
	}

	private void update() throws Exception {
		System.out.println("update");
		canvas.update();
		for (List<CellBlock> list : board) {
			for (CellBlock btn : list) {
				btn.setText(String.valueOf(btn.cell.count()));
				if (btn.cell.get_state()) {
					// + make it clicked initially
					btn.on();
				} else {
					// + make it released initially
					btn.off();
				}
			}
		}
	}
	
}

class CellBlock extends JButton {
	private static final long serialVersionUID = 8617456729265392099L;

	Cell cell;
	
	CellBlock () {
		this(null);
	}

	CellBlock (Cell cell) {
		if (cell != null) {
			this.cell = cell;
			addMouseListener(new MouseAdapter() {
				@Override
				public void mouseClicked(MouseEvent arg0) {
					// System.out.println(cell.get_state());
					if (cell.get_state()) {
						off();
						cell.update(false);
					} else {
						on();
						cell.update(true);
					}
				}
			});
		}
	}

	public void on() {
		setForeground(Color.BLACK);
		setBackground(Color.WHITE);
	}

	public void off() {
		setForeground(Color.WHITE);
		setBackground(Color.BLACK);
	}
}
