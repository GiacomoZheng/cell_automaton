pub trait Handle {
	type Item;
	fn handle(self, err_msg : &'static str) -> Result<Self::Item, &'static str>;
}

impl<T> Handle for Option<T> {
	type Item = T;
	fn handle(self, err_msg : &'static str) -> Result<Self::Item, &'static str> {
		if let Some(val) = self {
			Ok(val)
		} else {
			Err(err_msg)
		}
	}
}

pub mod screen;