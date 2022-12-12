use std::i32;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::cmp::PartialEq;

#[derive(Eq)]
pub struct Literal {
	id: i32,
}

impl Literal {

	pub fn new(int:i32) -> Literal {
		Literal {
			id: int,
		}
	}

	pub fn id(&self) -> i32 {
		self.id.abs()
	}


	pub fn sign(&self) -> i8 {
		if self.id >= 0 {
			1
		}
		else {
			-1
		}
	}


	pub fn is_positive(&self) -> bool {
		self.id >= 0
	}


	pub fn is_negative(&self) -> bool {
		self.id < 0
	}


	// Same id but opposite signs
	pub fn is_opposite(&self, other:&Self) -> bool {
		(self.id() == other.id()) && (self.sign() * other.sign() < 0)
	}
}


impl Ord for Literal {
	fn cmp(&self, other: &Literal) -> Ordering {
		self.id.cmp(&other.id)
	}
}


impl PartialOrd for Literal {
	fn partial_cmp(&self, other: &Literal) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}


impl PartialEq for Literal {
	fn eq(&self, other: &Literal) -> bool {
		self.id == other.id
	}
}


#[cfg(test)]
mod tests {
	use super::Literal;

	#[test]
	fn positive_literal() {
		let l = Literal::new(1);
		assert_eq!(1, l.id());
		assert_eq!(1, l.sign());
		assert_eq!(true, l.is_positive());
		assert_eq!(false, l.is_negative());
	}

	#[test]
	fn negative_literal() {
		let l = Literal::new(-2);
		assert_eq!(2, l.id());
		assert_eq!(-1, l.sign());
		assert_eq!(false, l.is_positive());
		assert_eq!(true, l.is_negative());
	}

	#[test]
	fn is_opposite() {
		let l1  = Literal::new(1);
		let ln1 = Literal::new(-1);
		let l22 = Literal::new(22);
		assert_eq!(true, l1.is_opposite(&ln1));
		assert_eq!(true, ln1.is_opposite(&l1));
		assert_eq!(false, l1.is_opposite(&l1));
		assert_eq!(false, ln1.is_opposite(&ln1));
		assert_eq!(false, l1.is_opposite(&l22));
	}
}
