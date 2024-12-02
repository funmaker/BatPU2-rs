use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct Char(u8);

impl Char {
	pub const TABLE: [char; 30] = [' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '.', '!', '?'];
	pub const SPACE: Char = Char(0);
	
	pub fn new(value: u8) -> Self {
		Self(value)
	}
	
	pub fn to_char(self) -> Option<char> {
		Char::TABLE.get(self.as_u8() as usize)
		           .copied()
	}
	
	pub fn as_u8(self) -> u8 {
		self.0
	}
	
	pub fn is_valid(self) -> bool {
		(self.as_u8() as usize) < Char::TABLE.len()
	}
}

impl TryFrom<char> for Char {
	type Error = String; // todo
	
	fn try_from(value: char) -> Result<Self, Self::Error> {
		if value == ' ' { Ok(Char::SPACE) }
		else if value == '.' { Ok(Char::new(27)) }
		else if value == '!' { Ok(Char::new(28)) }
		else if value == '?' { Ok(Char::new(29)) }
		else if value.is_ascii_alphabetic() {
			Ok(Char::new(value.to_ascii_uppercase() as u8 - 'A' as u8 + 1))
		}else{
			Err(format!("invalid char: {}", value))
		}
	}
}

impl Display for Char {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(c) = self.to_char() {
			c.fmt(f)
		} else {
			write!(f, "<{}>", self.0)
		}
	}
}
