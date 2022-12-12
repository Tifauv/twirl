use std::i32;
use literal::Literal;

pub struct Clause {
	id: i32,
	literals: Vec<Literal>,
	unsatisfiable: bool,
}


impl Clause {

	pub fn new(clause_id:i32) -> Clause {
		Clause {
			id: clause_id,
			literals: Vec::new(),
			unsatisfiable: false,
		}
	}


	pub fn id(self:&Self) -> i32 {
		self.id
	}


	pub fn add(&mut self, literal:Literal) {
		self.literals.push(literal)
	}


	pub fn remove(&mut self, literal:&Literal) {
		match self.literals.binary_search(literal) {
			Ok(index) => Some(self.literals.remove(index)),
			Err(_)    => None
		};
	}


	pub fn is_unsatisfiable(&self) -> bool {
		self.unsatisfiable
	}


	pub fn is_unary(&self) -> bool {
		self.literals.len() == 1
	}
}

