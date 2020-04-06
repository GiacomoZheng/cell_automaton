use rust_one::Canvas;
use rust_one::Rule;

use rust_one::tools::screen::MainView;
use orbtk::prelude::*;
use orbtk;
use orbtk::theme::DEFAULT_THEME_CSS;

static CSS_EXT: &'static str = include_str!("../res/grid.css");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(CSS_EXT)
        .build()
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - grid example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .theme(get_theme())
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}


/* #[allow(unused_variables)]
fn main() -> Result<(), &'static str> {
	const T : bool = true;
	const F : bool = false;

	let to_dead = Rule::from(vec![F, F, F, F, F], vec![F, F, F, F, F]);
		let lf = &to_dead.off();
		let pn = &to_dead.on();
	let to_live = Rule::from(vec![T, T, T, T, T], vec![T, T, T, T, T]);
		let ln = &to_live.on();
		let pf = &to_live.off();

	let to_self = Rule::from(vec![T, T, T, T, T], vec![F, F, F, F, F]);
		let sn = &to_self.on();
		let sf = &to_self.off();
		
	let b13_s13 = Rule::from(vec![F, T, F, T, F], vec![F, T, F, T, F]);
		let n13 = &b13_s13.on();
		let f13 = &b13_s13.off();

	let mut canvas = Canvas::new(8, 3);
	canvas.buildup_rect_board();
	canvas.init(vec![
		 lf, f13,  lf, f13,  lf, f13,  lf,  lf,
		 sn, f13, f13, f13, f13, f13, f13, f13,
		 lf,  lf, f13,  lf, f13,  lf, f13,  lf,
	])?;

	let mut generation = 0;
	loop {
		// println!("generation:{}\n{:?}", generation, canvas);
		println!("generation:{}\n{}", generation, canvas);

		// println!("{:?}", to_dead);
		// println!("{:?}", to_live);
		// println!("{:?}", to_self);
		println!("{:?}", b13_s13);
		
		println!("next:");

		std::io::stdin().read_line(&mut String::new()).unwrap();

		generation += 1;
		canvas.update()?;
	}

	#[allow(unreachable_code)]
	Ok(())

}
 */