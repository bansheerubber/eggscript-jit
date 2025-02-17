#[derive(Debug)]
pub struct Span {
	start: u32,
	end: u32,
}

impl Span {
	pub fn new(start: u32, end: u32) -> Span {
		Span { start, end }
	}

	pub fn start(&self) -> u32 {
		self.start
	}

	pub fn end(&self) -> u32 {
		self.end
	}
}
