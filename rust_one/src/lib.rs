#[cfg(test)]
mod test;

mod cell;
pub use crate::cell::{Cell, On, Off};

mod tools;
pub use crate::tools::{buildup_square_board, buildup_rect_board};

mod rules;
pub use crate::rules::Rule;