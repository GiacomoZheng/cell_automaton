use rust_one::Canvas;
use rust_one::Rule;

#[allow(unused_variables)]
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
	if let Err(e) = canvas.init(vec![
		 lf, f13,  lf, f13,  lf, f13,  lf,  lf,
		 sn, f13, f13, f13, f13, f13, f13, f13,
		 lf,  lf, f13,  lf, f13,  lf, f13,  lf,
	]) {
		eprintln!("{}", e);
	}

	let mut generation = 0;
	loop {
		eprintln!("generation:{}\n{:?}", generation, canvas);
		// eprintln!("generation:{}\n{}", generation, canvas);

		// eprintln!("{:?}", to_dead);
		// eprintln!("{:?}", to_live);
		// eprintln!("{:?}", to_self);
		eprintln!("{:?}", b13_s13);
		
		eprintln!("next:");

		std::io::stdin().read_line(&mut String::new()).unwrap();

		generation += 1;
		canvas.update()?;
	}

	#[allow(unreachable_code)]
	Ok(())

}
