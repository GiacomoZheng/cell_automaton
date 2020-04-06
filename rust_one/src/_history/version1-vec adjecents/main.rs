use rust_one::Canvas;
use rust_one::Rule;

fn main() {
	const T : bool = true;
	const F : bool = false;

	let longdead = Rule::from(vec![F, F, F, F, F], vec![F, F, F, F, F]);
		let saw_lf = longdead.off();
		let lf = Some(&saw_lf);
	let longlive = Rule::from(vec![T, T, T, T, T], vec![T, T, T, T, T]);
		let saw_ln = longlive.on();
		let ln = Some(&saw_ln);

	let mut canvas = Canvas::new(5, 4);
	canvas.buildup_rect_board();
	if let Err(e) = canvas.init(vec![
		vec![ lf, lf, lf, lf, lf ],
		vec![ lf, lf, ln, lf, lf ],
		vec![ lf, lf, lf, lf, lf ],
		vec![ lf, lf, lf, lf, lf ],
	]) {
		println!("{}", e);
	}
	println!("{:?}", canvas);
	println!("longdead::on  : {}", longdead.count(F));
	println!("longlive::off : {}", longlive.count(T));

	// test
	// let z = vec![1,2,3, 4];
	// let v = vec![Some(1), Some(2), None, Some(4)];
	// for i in v.iter().zip(z) {
	// 	println!("{:?}", i);
	// }
}
