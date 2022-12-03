use cell_automation::Canvas;
use cell_automation::{CellAutomaton};
use cell_automation::Rule;

use cell_automation::{Rect};

use std::time::{SystemTime};

// /* 
#[allow(unused_variables)]
fn main() -> Result<(), &'static str> {
	// const T : bool = true;
	// const F : bool = false;

	let to_off = Rule::to_off(5);
		let _lf = &to_off.off();
		let _pn = &to_off.on();
	let to_on = Rule::to_on(5);
		let _ln = &to_on.on();
		let _pf = &to_on.off();

	let to_self = Rule::to_self(5);
		let _sn = &to_self.on();
		let _sf = &to_self.off();
	let strobe = Rule::to_opp(5);
		#[allow(non_snake_case)]
		let nSt = &strobe.on();
		#[allow(non_snake_case)]
		let fSt = &strobe.off();
		
	let b135_s135 = Rule::from_B_S_C("135", "135", 5); // Rule::from(vec![F, T, F, T, F], vec![F, T, F, T, F]);
		let odn = &b135_s135.on();
		let odf = &b135_s135.off();

	let b1_s1 = Rule::from_B_S_C("1", "1", 1);
		let n_1 = &b1_s1.on();
		let f_1 = &b1_s1.off();

	let b24_s24 = Rule::from_B_S_C("24", "24", 4);
		let n24 = &b24_s24.on();
		let f24 = &b24_s24.off();

	#[allow(non_snake_case)]
	let __N = &Rule::none();

	let sy_time = SystemTime::now();

	// /* 
	let mut canvas = CellAutomaton::new(3, 3, 2);
	canvas.set_type(Rect);
	canvas.buildup_board();
	canvas.init(vec![
		_lf, _lf, _lf,
		_lf, nSt, _lf,
		_lf, _lf, _lf,

		// screen part
		__N, __N, __N,
		__N, odf, __N,
		__N, __N, __N,
	]);
	// */

	/*
	let mut canvas = Canvas::new(8, 3, 2);
	canvas.set_type(Rect);
	canvas.buildup_board();
	canvas.init(vec![
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_sn, odf, odf, odf, odf, odf, odf, odf,
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		// screen part
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		odf, odf, odf, odf, odf, odf, odf, _lf,
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
	]);
	// */


	/* 
	let mut canvas = Canvas::new(16, 16, 2);
	canvas.set_type(Rect);
	canvas.buildup_board();
	canvas.init(vec![
		// + I want to print a  "8"
		_lf, _lf, f_1, _lf, f_1, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, _sn, odf, odf, odf, odf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		f_1, odf, _lf, _lf, _lf, odf, f_1, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, odf, _lf, f_1, _lf, odf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		f_1, odf, _lf, odf, odf, odf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, odf, _lf, odf, f24, odf, f_1, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		f_1, odf, _lf, _lf, odf, odf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, odf, _lf, _lf, _lf, odf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		f_1, odf, _lf, _lf, _lf, odf, f_1, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, odf, odf, odf, _lf, odf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, _lf, odf, _lf, _lf, f_1, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		_lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf, _lf,
		 // screen
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
		__N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N, __N,
		__N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1, __N, f_1,
	]);
	// */

	// eprintln!("{:?}", sy_time.elapsed().unwrap().as_millis());

	let mut generation = 0;
	loop {
		if generation >= 1000 {break;}
		
		eprintln!("# of need to update: {:?}", canvas.count_updating());
		
		println!("generation: {}\n{:?}", generation, canvas);
		// println!("generation:{}\n{}", generation, canvas);
		
		// println!("{:?}", to_off);
		// println!("{:?}", to_on);
		// println!("{:?}", to_self);
		// println!("{:?}", b135_s135);
		
		println!("next:");
		
		std::io::stdin().read_line(&mut String::new()).unwrap();
		
		generation += 1;
		canvas.update()?;
	}

	// #[allow(unreachable_code)]
	
	eprintln!("{:?}", sy_time.elapsed().unwrap().as_millis());
	println!("generation:{}\n{}", generation, canvas);

	Ok(())

}
//  */