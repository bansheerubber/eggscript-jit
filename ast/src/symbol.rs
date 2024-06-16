pub type Symbol = String;

#[derive(Clone, Copy, Debug)]
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

impl From<pest::Span<'_>> for Span {
	fn from(value: pest::Span) -> Self {
		Span {
			start: value.start() as u32,
			end: value.end() as u32,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Ident {
	name: Symbol,
	span: Span,
}

impl Ident {
	pub fn new(name: &str, span: Span) -> Ident {
		Ident {
			name: name.into(),
			span,
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}
