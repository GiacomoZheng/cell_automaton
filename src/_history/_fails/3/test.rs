use super::*;

#[test]
fn rule() {
	let dead = Off::from(vec![false, false, false, false]); // for dead cell, we don't need to consider the turn  on case
	let live =  On::from(vec![ true,  true,  true,  true]); // for live cell, we don't need to consider the turn off case
}